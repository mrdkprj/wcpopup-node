use async_std::sync::Mutex;
#[cfg(target_os = "linux")]
use gdkx11::ffi::{gdk_x11_get_default_xdisplay, gdk_x11_lookup_xdisplay, gdk_x11_window_foreign_new_for_display};
#[cfg(target_os = "linux")]
use gtk::{
    gdk::Window,
    glib::{self, translate::FromGlibPtrFull},
    prelude::WidgetExt,
};
use neon::{
    handle::Handle,
    prelude::{Context, FunctionContext, ModuleContext},
    result::{JsResult, NeonResult},
    types::{JsArray, JsNumber, JsObject, JsPromise, JsString, JsUndefined, JsValue},
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use wcpopup::{
    config::{Config, Theme},
    Menu, MenuBuilder, MenuIcon,
};
mod types;
use types::*;

static MENU_MAP: Lazy<Mutex<HashMap<i32, Menu>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn build_from_template(mut cx: FunctionContext) -> JsResult<JsNumber> {
    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let parent = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let templates = cx.argument::<JsArray>(1)?.to_vec(&mut cx).unwrap();

    let menu_handle = build(&mut cx, parent, templates, Config::default());
    let id = cx.number(menu_handle as i32);
    Ok(id)
}

pub fn build_from_template_with_theme(mut cx: FunctionContext) -> JsResult<JsNumber> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let parent = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let templates = cx.argument::<JsArray>(1)?.to_vec(&mut cx).unwrap();
    let theme_str = cx.argument::<JsString>(2)?.value(&mut cx);

    let theme = match theme_str.as_str() {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        "system" => Theme::System,
        _ => Theme::System,
    };

    let config = Config {
        theme,
        ..Default::default()
    };

    let menu_handle = build(&mut cx, parent, templates, config);
    let id = cx.number(menu_handle as i32);
    Ok(id)
}

pub fn build_from_template_with_config(mut cx: FunctionContext) -> JsResult<JsNumber> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let parent = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let templates = cx.argument::<JsArray>(1)?.to_vec(&mut cx).unwrap();
    let config_obj = cx.argument::<JsObject>(2)?;

    let config = to_config(&mut cx, config_obj);

    let menu_handle = build(&mut cx, parent, templates, config);

    let id = cx.number(menu_handle as i32);
    Ok(id)
}

fn build(cx: &mut FunctionContext, parent: f64, templates: Vec<Handle<JsValue>>, config: Config) -> isize {
    let items: Vec<ElectronMenuItem> = templates
        .into_iter()
        .map(|value| {
            let v = value.downcast_or_throw::<JsObject, _>(cx).unwrap();
            ElectronMenuItem::from_object(cx, v)
        })
        .collect();

    #[cfg(target_os = "linux")]
    {
        let _ = gtk::init();
        let display = unsafe { gdk_x11_get_default_xdisplay() };

        let x11_display = unsafe { gdk_x11_lookup_xdisplay(display) };
        let gdkwindow = unsafe { gdk_x11_window_foreign_new_for_display(x11_display, parent as u64) };
        let gdk_window: Window = unsafe { Window::from_glib_full(gdkwindow) };
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.connect_realize(glib::clone!(@weak gdk_window as wd => move |w| w.set_window(wd)));
        window.set_has_window(true);
        window.realize();

        let mut builder = MenuBuilder::new_for_window_from_config(&window, config);
        build_menu(&mut builder, &items);
        let menu = builder.build().unwrap();
        let mut map = MENU_MAP.try_lock().unwrap();

        let menu_handle = menu.gtk_menu_handle;
        (*map).insert(menu_handle as i32, menu);

        menu_handle
    }

    #[cfg(target_os = "windows")]
    {
        let mut builder = MenuBuilder::new_from_config(parent as isize, config);

        build_menu(&mut builder, &items);
        let menu = builder.build().unwrap();

        let mut map = MENU_MAP.try_lock().unwrap();

        let menu_handle = menu.window_handle;
        (*map).insert(menu_handle as i32, menu);

        menu_handle
    }
}

fn build_menu(builder: &mut MenuBuilder, items: &Vec<ElectronMenuItem>) {
    for item in items {
        let disabled = !item.enabled;
        match item.itype.as_str() {
            "normal" => {
                if item.accelerator.is_empty() && item.icon.is_empty() {
                    builder.text(&item.id, &item.label, disabled);
                } else if item.icon.is_empty() {
                    builder.text_with_accelerator(&item.id, &item.label, disabled, &item.accelerator);
                } else {
                    builder.text_with_accel_icon(&item.id, &item.label, disabled, &item.accelerator, MenuIcon::new(&item.icon));
                }
            }
            "separator" => {
                builder.separator();
            }
            "submenu" => {
                let mut parent = if item.icon.is_empty() {
                    builder.submenu(&item.id, &item.label, disabled)
                } else {
                    builder.submenu_with_icon(&item.id, &item.label, disabled, MenuIcon::new(&item.icon))
                };
                build_menu(&mut parent, &item.submenu);
                let submenu = parent.build().unwrap();
                let mut map = MENU_MAP.try_lock().unwrap();
                #[cfg(target_os = "linux")]
                (*map).insert(submenu.gtk_menu_handle as i32, submenu);
                #[cfg(target_os = "windows")]
                (*map).insert(submenu.window_handle as i32, submenu);
                std::mem::drop(map);
            }
            "checkbox" => {
                if item.accelerator.is_empty() {
                    builder.check(&item.id, &item.label, item.checked, disabled);
                } else {
                    builder.check_with_accelerator(&item.id, &item.label, item.checked, disabled, &item.accelerator);
                }
            }
            "radio" => {
                if item.accelerator.is_empty() {
                    builder.radio(&item.id, &item.label, &item.name, item.checked, disabled);
                } else {
                    builder.radio_with_accelerator(&item.id, &item.label, &item.name, item.checked, disabled, &item.accelerator);
                }
            }
            _ => {}
        }
    }
}

pub fn popup(mut cx: FunctionContext) -> JsResult<JsPromise> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let menu_handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let x = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(2)?.value(&mut cx);

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();

    #[cfg(target_os = "windows")]
    async_std::task::spawn(async move {
        let map = MENU_MAP.lock().await;
        let menu = map.get(&(menu_handle as i32)).unwrap();
        let x = menu.popup_at_async(x as i32, y as i32).await;

        deferred.settle_with(&channel, |mut cx| match x {
            Some(data) => from_menu_item(&mut cx, &data),
            None => Ok(cx.empty_object()),
        });
    });
    #[cfg(target_os = "linux")]
    gtk::glib::spawn_future_local(async move {
        let map = MENU_MAP.lock().await;
        let menu = map.get(&(menu_handle as i32)).unwrap();
        let x = menu.popup_at_async(x as i32, y as i32).await;
        deferred.settle_with(&channel, |mut cx| match x {
            Some(data) => from_menu_item(&mut cx, &data),
            None => Ok(cx.empty_object()),
        });
    });

    Ok(promise)
}

pub fn items(mut cx: FunctionContext) -> JsResult<JsArray> {
    let menu_handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let map = MENU_MAP.try_lock().unwrap();
    let menu = map.get(&(menu_handle as i32)).unwrap();
    let items = extract_item(&menu.items(), &mut cx)?;

    Ok(items)
}

pub fn remove(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let menu_handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(1)?;

    let item = to_menu_item(&mut cx, jsitem);

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(menu_handle as i32)).unwrap();
    menu.remove(&item);

    Ok(cx.undefined())
}

pub fn remove_at(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let menu_handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let index = cx.argument::<JsNumber>(1)?.value(&mut cx);

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(menu_handle as i32)).unwrap();
    menu.remove_at(index as u32);

    Ok(cx.undefined())
}

pub fn append(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let menu_handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(1)?;

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(menu_handle as i32)).unwrap();
    let item = to_menu_item(&mut cx, jsitem);
    menu.append(item);

    Ok(cx.undefined())
}

pub fn insert(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let menu_handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let index = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(2)?;

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(menu_handle as i32)).unwrap();
    let item = to_menu_item(&mut cx, jsitem);
    menu.insert(item, index as u32);

    Ok(cx.undefined())
}

pub fn get_menu_item_by_id(mut cx: FunctionContext) -> JsResult<JsObject> {
    let menu_handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let id = cx.argument::<JsString>(1)?.value(&mut cx);

    let map = MENU_MAP.try_lock().unwrap();
    let menu = map.get(&(menu_handle as i32)).unwrap();
    if let Some(item) = menu.get_menu_item_by_id(id.as_str()) {
        from_menu_item(&mut cx, &item)
    } else {
        Ok(cx.empty_object())
    }
}

pub fn get_default_config(mut cx: FunctionContext) -> JsResult<JsObject> {
    let configjs = from_config(&mut cx, &Config::default())?;
    Ok(configjs)
}

pub fn set_theme(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let theme_str = cx.argument::<JsString>(1)?.value(&mut cx);

    let theme = match theme_str.as_str() {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        "system" => Theme::System,
        _ => Theme::System,
    };

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(id as i32)).unwrap();
    menu.set_theme(theme);

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("buildFromTemplate", build_from_template)?;
    cx.export_function("buildFromTemplateWithTheme", build_from_template_with_theme)?;
    cx.export_function("buildFromTemplateWithConfig", build_from_template_with_config)?;

    cx.export_function("setTheme", set_theme)?;
    cx.export_function("items", items)?;
    cx.export_function("removeAt", remove_at)?;
    cx.export_function("remove", remove)?;
    cx.export_function("append", append)?;
    cx.export_function("insert", insert)?;
    cx.export_function("getMenuItemById", get_menu_item_by_id)?;
    cx.export_function("popup", popup)?;

    cx.export_function("getDefaultConfig", get_default_config)?;

    Ok(())
}
