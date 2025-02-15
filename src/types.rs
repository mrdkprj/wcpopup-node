use std::path::PathBuf;

use neon::{
    handle::Handle,
    prelude::*,
    result::JsResult,
    types::{JsArray, JsBoolean, JsNumber, JsObject, JsString},
};
use wcpopup::{
    config::{ColorScheme, Config, Corner, FontWeight, IconSettings, MenuFont, MenuSVG, MenuSize, Theme, ThemeColor},
    Menu, MenuIcon, MenuItem, MenuItemType, MenuType,
};

#[derive(Debug, Clone)]
pub struct ElectronMenuItem {
    pub itype: String,
    pub label: String,
    pub accelerator: String,
    pub enabled: bool,
    pub checked: bool,
    pub submenu: Vec<ElectronMenuItem>,
    pub id: String,
    pub name: String,
    pub icon: String,
}

impl ElectronMenuItem {
    pub fn from_object(cx: &mut FunctionContext, value: Handle<JsObject>) -> Self {
        Self {
            itype: to_string(cx, &value, "type"),
            label: to_string(cx, &value, "label"),
            accelerator: to_string(cx, &value, "accelerator"),
            enabled: to_bool(cx, &value, "enabled", true),
            checked: to_bool(cx, &value, "checked", false),
            submenu: value
                .get_opt::<JsArray, _, _>(cx, "submenu")
                .unwrap()
                .unwrap_or_else(|| JsArray::new(cx, 0))
                .to_vec(cx)
                .unwrap()
                .into_iter()
                .map(|value| {
                    let v = value.downcast_or_throw::<JsObject, _>(cx).unwrap();

                    ElectronMenuItem::from_object(cx, v)
                })
                .collect(),
            id: to_string(cx, &value, "id"),
            name: to_string(cx, &value, "name"),
            icon: to_string(cx, &value, "icon"),
        }
    }
}

pub fn to_string(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str) -> String {
    value.get_opt::<JsString, _, _>(cx, key).unwrap().unwrap_or_else(|| JsString::new(cx, "")).value(cx)
}

pub fn to_bool(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str, def: bool) -> bool {
    value.get_opt::<JsBoolean, _, _>(cx, key).unwrap().unwrap_or_else(|| JsBoolean::new(cx, def)).value(cx)
}

pub fn to_i32(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str) -> i32 {
    value.get_opt::<JsNumber, _, _>(cx, key).unwrap().unwrap_or_else(|| JsNumber::new(cx, 0)).value(cx) as i32
}

pub fn to_u32(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str) -> u32 {
    value.get_opt::<JsNumber, _, _>(cx, key).unwrap().unwrap_or_else(|| JsNumber::new(cx, 0)).value(cx) as u32
}

pub fn to_f32(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str) -> f32 {
    value.get_opt::<JsNumber, _, _>(cx, key).unwrap().unwrap_or_else(|| JsNumber::new(cx, 0)).value(cx) as f32
}

pub fn to_menu_item(cx: &mut FunctionContext, value: Handle<JsObject>) -> MenuItem {
    let id = to_string(cx, &value, "id");
    let label = to_string(cx, &value, "label");
    let accelerator_str = to_string(cx, &value, "accelerator");
    let name = to_string(cx, &value, "name");
    let enabled = to_bool(cx, &value, "enabled", true);
    let checked = to_bool(cx, &value, "checked", false);
    let icon_path = to_string(cx, &value, "icon");
    let icon = if icon_path.is_empty() {
        None
    } else {
        Some(MenuIcon::new(icon_path))
    };

    let accelerator = if accelerator_str.is_empty() {
        None
    } else {
        Some(accelerator_str.as_str())
    };
    let disabled = !enabled;

    let item_type_str = to_string(cx, &value, "type");

    let menu_item_type = match item_type_str.as_str() {
        "normal" => MenuItemType::Text,
        "separator" => MenuItemType::Separator,
        "submenu" => MenuItemType::Submenu,
        "checkbox" => MenuItemType::Checkbox,
        "radio" => MenuItemType::Radio,
        _ => MenuItemType::Text,
    };

    let mut item = match menu_item_type {
        MenuItemType::Text => MenuItem::new_text_item(&id, &label, accelerator, disabled, icon),
        MenuItemType::Separator => MenuItem::new_separator(),
        MenuItemType::Submenu => MenuItem::new_text_item(&id, &label, accelerator, disabled, icon),
        MenuItemType::Checkbox => MenuItem::new_check_item(&id, &label, accelerator, checked, disabled, icon),
        MenuItemType::Radio => MenuItem::new_radio_item(&id, &label, &name, accelerator, checked, disabled, icon),
    };

    item.uuid = to_i32(cx, &value, "uuid") as u16;
    item
}

pub fn extract_item<'a, C: Context<'a>>(vec: &[MenuItem], cx: &mut C) -> JsResult<'a, JsArray> {
    let items = JsArray::new(cx, vec.len());
    for (index, item) in vec.iter().enumerate() {
        if item.menu_item_type == MenuItemType::Submenu {
            let obj = from_menu_item(cx, item)?;
            items.set(cx, index as u32, obj)?;
            let submenus = item.submenu.as_ref().unwrap().items();
            extract_item(&submenus, cx)?;
        } else {
            let obj = from_menu_item(cx, item)?;
            items.set(cx, index as u32, obj)?;
        }
    }

    Ok(items)
}

pub fn from_menu_item<'a, C: Context<'a>>(cx: &mut C, item: &MenuItem) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let id = cx.string(item.id.clone());
    obj.set(cx, "id", id)?;

    let label = cx.string(item.label.clone());
    obj.set(cx, "label", label)?;

    let accelerator = cx.string(item.accelerator.clone());
    obj.set(cx, "accelerator", accelerator)?;

    let name = cx.string(item.name.clone());
    obj.set(cx, "name", name)?;

    let checked = cx.boolean(item.checked);
    obj.set(cx, "checked", checked)?;

    let enabled = cx.boolean(!item.disabled);
    obj.set(cx, "enabled", enabled)?;

    let uuid = cx.number(item.uuid);
    obj.set(cx, "uuid", uuid)?;

    let menu_item_type_str = match item.menu_item_type {
        MenuItemType::Text => "normal",
        MenuItemType::Separator => "separator",
        MenuItemType::Submenu => "submenu",
        MenuItemType::Checkbox => "checkbox",
        MenuItemType::Radio => "radio",
    };
    let menu_item_type_str = cx.string(menu_item_type_str);
    obj.set(cx, "type", menu_item_type_str)?;

    let submenu = if item.submenu.is_some() {
        from_menu(cx, item.submenu.as_ref().unwrap())?
    } else {
        cx.empty_object()
    };
    obj.set(cx, "submenu", submenu)?;

    Ok(obj)
}

pub fn from_menu<'a, C: Context<'a>>(cx: &mut C, menu: &Menu) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    #[cfg(target_os = "windows")]
    let menu_handle = cx.number(menu.window_handle as f64);
    #[cfg(target_os = "linux")]
    let menu_handle = cx.number(menu.gtk_menu_handle as f64);
    obj.set(cx, "menuWindowHandle", menu_handle)?;

    let type_str = match menu.menu_type {
        MenuType::Main => "main",
        MenuType::Submenu => "submenu",
    };
    let menu_type = cx.string(type_str);
    obj.set(cx, "type", menu_type)?;

    Ok(obj)
}

pub fn to_config(cx: &mut FunctionContext, value: Handle<JsObject>) -> Config {
    let theme = match to_string(cx, &value, "theme").as_str() {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        "system" => Theme::System,
        _ => Theme::System,
    };

    let size_obj = value.get::<JsObject, _, _>(cx, "size").unwrap();

    let size = MenuSize {
        border_size: to_i32(cx, &size_obj, "borderSize"),
        vertical_padding: to_i32(cx, &size_obj, "verticalPadding"),
        horizontal_padding: to_i32(cx, &size_obj, "horizontalPadding"),
        item_vertical_padding: to_i32(cx, &size_obj, "itemVerticalPadding"),
        item_horizontal_padding: to_i32(cx, &size_obj, "itemHorizontalPadding"),
        submenu_offset: to_i32(cx, &size_obj, "submenuOffset"),
        separator_size: to_i32(cx, &size_obj, "separatorSize"),
    };

    let font_obj = value.get::<JsObject, _, _>(cx, "font").unwrap();
    let font = MenuFont {
        font_family: to_string(cx, &font_obj, "fontFamily"),
        dark_font_size: to_f32(cx, &font_obj, "darkFontSize"),
        dark_font_weight: match to_string(cx, &font_obj, "darkFontWeight").as_str() {
            "Thin" => FontWeight::Thin,
            "Light" => FontWeight::Light,
            "Normal" => FontWeight::Normal,
            "Medium" => FontWeight::Medium,
            "Bold" => FontWeight::Bold,
            _ => FontWeight::Normal,
        },
        light_font_size: to_f32(cx, &font_obj, "lightFontSize"),
        light_font_weight: match to_string(cx, &font_obj, "lightFontWeight").as_str() {
            "Thin" => FontWeight::Thin,
            "Light" => FontWeight::Light,
            "Normal" => FontWeight::Normal,
            "Medium" => FontWeight::Medium,
            "Bold" => FontWeight::Bold,
            _ => FontWeight::Normal,
        },
    };

    let color_obj = value.get::<JsObject, _, _>(cx, "color").unwrap();
    let dark_color_scheme_obj = color_obj.get::<JsObject, _, _>(cx, "dark").unwrap();
    let light_color_scheme_obj = color_obj.get::<JsObject, _, _>(cx, "light").unwrap();

    let dark = ColorScheme {
        color: to_u32(cx, &dark_color_scheme_obj, "color"),
        accelerator: to_u32(cx, &dark_color_scheme_obj, "accelerator"),
        border: to_u32(cx, &dark_color_scheme_obj, "border"),
        separator: to_u32(cx, &dark_color_scheme_obj, "separator"),
        disabled: to_u32(cx, &dark_color_scheme_obj, "disabled"),
        background_color: to_u32(cx, &dark_color_scheme_obj, "backgroundColor"),
        hover_background_color: to_u32(cx, &dark_color_scheme_obj, "hoverBackgroundColor"),
    };

    let light = ColorScheme {
        color: to_u32(cx, &light_color_scheme_obj, "color"),
        accelerator: to_u32(cx, &light_color_scheme_obj, "accelerator"),
        border: to_u32(cx, &light_color_scheme_obj, "border"),
        separator: to_u32(cx, &light_color_scheme_obj, "separator"),
        disabled: to_u32(cx, &light_color_scheme_obj, "disabled"),
        background_color: to_u32(cx, &light_color_scheme_obj, "backgroundColor"),
        hover_background_color: to_u32(cx, &light_color_scheme_obj, "hoverBackgroundColor"),
    };

    let color = ThemeColor {
        dark,
        light,
    };

    let corner = if to_string(cx, &value, "corner") == "Round" {
        Corner::Round
    } else {
        Corner::DoNotRound
    };

    let icon_obj = value.get_opt::<JsObject, _, _>(cx, "icon").unwrap();
    let icon = if let Some(icon_obj) = icon_obj {
        let check_svg_obj = icon_obj.get_opt::<JsObject, _, _>(cx, "checkSVG").unwrap();
        let check_svg = check_svg_obj.map(|chekc_svg| MenuSVG {
            path: PathBuf::from(to_string(cx, &chekc_svg, "path")),
            width: to_i32(cx, &chekc_svg, "width"),
            height: to_i32(cx, &chekc_svg, "height"),
        });

        let arrow_svg_obj = icon_obj.get_opt::<JsObject, _, _>(cx, "arrowSVG").unwrap();
        let arrow_svg = arrow_svg_obj.map(|arrow_svg| MenuSVG {
            path: PathBuf::from(to_string(cx, &arrow_svg, "path")),
            width: to_i32(cx, &arrow_svg, "width"),
            height: to_i32(cx, &arrow_svg, "height"),
        });

        Some(IconSettings {
            check_svg,
            arrow_svg,
            reserve_icon_size: to_bool(cx, &icon_obj, "reserveIconSize", false),
            horizontal_margin: Some(to_i32(cx, &icon_obj, "horizontalMargin")),
        })
    } else {
        None
    };

    Config {
        theme,
        size,
        color,
        corner,
        font,
        icon,
    }
}

pub fn from_config<'a, C: Context<'a>>(cx: &mut C, config: &Config) -> JsResult<'a, JsObject> {
    let configjs = cx.empty_object();

    let theme_str = match config.theme {
        Theme::Dark => "dark",
        Theme::Light => "light",
        Theme::System => "system",
    };
    let theme = cx.string(theme_str);

    configjs.set(cx, "theme", theme)?;

    let size = cx.empty_object();
    let a = cx.number(config.size.border_size);
    size.set(cx, "borderSize", a)?;
    let a = cx.number(config.size.vertical_padding);
    size.set(cx, "verticalPadding", a)?;
    let a = cx.number(config.size.horizontal_padding);
    size.set(cx, "horizontalPadding", a)?;
    let a = cx.number(config.size.item_vertical_padding);
    size.set(cx, "itemVerticalPadding", a)?;
    let a = cx.number(config.size.item_horizontal_padding);
    size.set(cx, "itemHorizontalPadding", a)?;
    let a = cx.number(config.size.submenu_offset);
    size.set(cx, "submenuOffset", a)?;

    configjs.set(cx, "size", size)?;

    let color = cx.empty_object();
    let dark = cx.empty_object();
    let a = cx.number(config.color.dark.color);
    dark.set(cx, "color", a)?;
    let a = cx.number(config.color.dark.accelerator);
    dark.set(cx, "accelerator", a)?;
    let a = cx.number(config.color.dark.border);
    dark.set(cx, "border", a)?;
    let a = cx.number(config.color.dark.separator);
    dark.set(cx, "separator", a)?;
    let a = cx.number(config.color.dark.disabled);
    dark.set(cx, "disabled", a)?;
    let a = cx.number(config.color.dark.background_color);
    dark.set(cx, "backgroundColor", a)?;
    let a = cx.number(config.color.dark.hover_background_color);
    dark.set(cx, "hoverBackgroundColor", a)?;
    color.set(cx, "dark", dark)?;

    let light = cx.empty_object();
    let a = cx.number(config.color.light.color);
    light.set(cx, "color", a)?;
    let a = cx.number(config.color.light.accelerator);
    light.set(cx, "accelerator", a)?;
    let a = cx.number(config.color.light.border);
    light.set(cx, "border", a)?;
    let a = cx.number(config.color.light.separator);
    light.set(cx, "separator", a)?;
    let a = cx.number(config.color.light.disabled);
    light.set(cx, "disabled", a)?;
    let a = cx.number(config.color.light.background_color);
    light.set(cx, "backgroundColor", a)?;
    let a = cx.number(config.color.light.hover_background_color);
    light.set(cx, "hoverBackgroundColor", a)?;
    color.set(cx, "light", light)?;

    configjs.set(cx, "color", color)?;

    let corner = cx.string(if config.corner == Corner::Round {
        "Round"
    } else {
        "DoNotRound"
    });

    configjs.set(cx, "corner", corner)?;

    let font = cx.empty_object();
    let a = cx.string(config.font.font_family.clone());
    font.set(cx, "fontFamily", a)?;
    let a = cx.number(config.font.dark_font_size);
    font.set(cx, "darkFontSize", a)?;
    let a = cx.string(match config.font.dark_font_weight {
        FontWeight::Thin => "Thin",
        FontWeight::Light => "Light",
        FontWeight::Normal => "Normal",
        FontWeight::Medium => "Medium",
        FontWeight::Bold => "Bold",
    });
    font.set(cx, "darkFontWeight", a)?;
    let a = cx.number(config.font.light_font_size);
    font.set(cx, "lightFontSize", a)?;
    let a = cx.string(match config.font.light_font_weight {
        FontWeight::Thin => "Thin",
        FontWeight::Light => "Light",
        FontWeight::Normal => "Normal",
        FontWeight::Medium => "Medium",
        FontWeight::Bold => "Bold",
    });
    font.set(cx, "lightFontWeight", a)?;

    configjs.set(cx, "font", font)?;

    let icon_obj = cx.empty_object();
    if let Some(icon) = &config.icon {
        let check_svg_obj = cx.empty_object();
        if let Some(check_svg) = &icon.check_svg {
            let a = cx.string(check_svg.path.to_string_lossy());
            check_svg_obj.set(cx, "path", a)?;
            let a = cx.number(check_svg.width);
            check_svg_obj.set(cx, "width", a)?;
            let a = cx.number(check_svg.height);
            check_svg_obj.set(cx, "height", a)?;
        }
        icon_obj.set(cx, "checkSVG", check_svg_obj)?;

        let arrow_svg_obj = cx.empty_object();
        if let Some(arrow_svg) = &icon.arrow_svg {
            let a = cx.string(arrow_svg.path.to_string_lossy());
            arrow_svg_obj.set(cx, "path", a)?;
            let a = cx.number(arrow_svg.width);
            arrow_svg_obj.set(cx, "width", a)?;
            let a = cx.number(arrow_svg.height);
            arrow_svg_obj.set(cx, "height", a)?;
        }
        icon_obj.set(cx, "checkSVG", arrow_svg_obj)?;
        let a = cx.boolean(icon.reserve_icon_size);
        icon_obj.set(cx, "reserveIconSize", a)?;
        let a = cx.number(icon.horizontal_margin.unwrap_or(0));
        icon_obj.set(cx, "horizontalMargin", a)?;
    }

    configjs.set(cx, "icon", icon_obj)?;

    Ok(configjs)
}
