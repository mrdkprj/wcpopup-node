use neon::{
    handle::Handle,
    prelude::*,
    result::JsResult,
    types::{JsArray, JsBoolean, JsNumber, JsObject, JsString},
};
use wcpopup::{ColorScheme, Config, Corner, FontWeight, Menu, MenuFont, MenuItem, MenuItemType, MenuSize, MenuType, Theme, ThemeColor};

#[derive(Debug)]
pub struct ElectronMenuItem {
    pub itype: String,
    pub label: String,
    pub accelerator: String,
    pub enabled: bool,
    pub checked: bool,
    pub submenu: Vec<ElectronMenuItem>,
    pub id: String,
    pub value: String,
    pub name: String,
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
            value: to_string(cx, &value, "value"),
            name: to_string(cx, &value, "name"),
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

pub fn to_f32(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str) -> f32 {
    value.get_opt::<JsNumber, _, _>(cx, key).unwrap().unwrap_or_else(|| JsNumber::new(cx, 0)).value(cx) as f32
}

pub fn to_menu_item(cx: &mut FunctionContext, value: Handle<JsObject>) -> MenuItem {
    let id = to_string(cx, &value, "id");
    let label = to_string(cx, &value, "label");
    let item_value = to_string(cx, &value, "value");
    let accelerator_str = to_string(cx, &value, "accelerator");
    let name = to_string(cx, &value, "name");
    let enabled = to_bool(cx, &value, "enabled", true);
    let checked = to_bool(cx, &value, "checked", false);

    let accelerator = if accelerator_str.is_empty() {
        None
    } else {
        Some(accelerator_str.as_str())
    };
    let disabled = if enabled {
        None
    } else {
        Some(true)
    };

    let item_type_str = to_string(cx, &value, "type");

    let menu_item_type = match item_type_str.as_str() {
        "normal" => MenuItemType::Text,
        "separator" => MenuItemType::Separator,
        "submenu" => MenuItemType::Submenu,
        "checkbox" => MenuItemType::Checkbox,
        "radio" => MenuItemType::Radio,
        _ => MenuItemType::Text,
    };

    match menu_item_type {
        MenuItemType::Text => MenuItem::new_text_item(&id, &label, &item_value, accelerator, disabled),
        MenuItemType::Separator => MenuItem::new_separator(),
        MenuItemType::Submenu => MenuItem::new_text_item(&id, &label, &item_value, accelerator, disabled),
        MenuItemType::Checkbox => MenuItem::new_check_item(&id, &label, &item_value, accelerator, checked, disabled),
        MenuItemType::Radio => MenuItem::new_radio_item(&id, &label, &item_value, &name, accelerator, checked, disabled),
    }
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

    let value = cx.string(item.value.clone());
    obj.set(cx, "value", value)?;

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

    let hwnd = cx.number(menu.hwnd.0 as f64);
    obj.set(cx, "hwnd", hwnd)?;

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
        horizontal_padding: to_i32(cx, &size_obj, "horizontalPaddint"),
        item_vertical_padding: to_i32(cx, &size_obj, "itemVerticalPadding"),
        item_horizontal_padding: to_i32(cx, &size_obj, "itemHorizontalPadding"),
        submenu_offset: to_i32(cx, &size_obj, "submenuOffset"),
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
        color: to_i32(cx, &dark_color_scheme_obj, "color") as u32,
        accelerator: to_i32(cx, &dark_color_scheme_obj, "accelerator") as u32,
        border: to_i32(cx, &dark_color_scheme_obj, "border") as u32,
        separator: to_i32(cx, &dark_color_scheme_obj, "separator") as u32,
        disabled: to_i32(cx, &dark_color_scheme_obj, "disabled") as u32,
        background_color: to_i32(cx, &dark_color_scheme_obj, "backgroundColor") as u32,
        hover_background_color: to_i32(cx, &dark_color_scheme_obj, "hoverBackgroundColor") as u32,
    };

    let light = ColorScheme {
        color: to_i32(cx, &light_color_scheme_obj, "color") as u32,
        accelerator: to_i32(cx, &light_color_scheme_obj, "accelerator") as u32,
        border: to_i32(cx, &light_color_scheme_obj, "border") as u32,
        separator: to_i32(cx, &light_color_scheme_obj, "separator") as u32,
        disabled: to_i32(cx, &light_color_scheme_obj, "disabled") as u32,
        background_color: to_i32(cx, &light_color_scheme_obj, "backgroundColor") as u32,
        hover_background_color: to_i32(cx, &light_color_scheme_obj, "hoverBackgroundColor") as u32,
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

    Config {
        theme,
        size,
        color,
        corner,
        font,
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
    size.set(cx, "fontFamily", a)?;
    let a = cx.number(config.font.dark_font_size);
    size.set(cx, "darkFontSize", a)?;
    let a = cx.string(match config.font.dark_font_weight {
        FontWeight::Thin => "Thin",
        FontWeight::Light => "Light",
        FontWeight::Normal => "Normal",
        FontWeight::Medium => "Medium",
        FontWeight::Bold => "Bold",
    });
    size.set(cx, "darkFontWeight", a)?;
    let a = cx.number(config.font.light_font_size);
    size.set(cx, "lightFontSize", a)?;
    let a = cx.string(match config.font.light_font_weight {
        FontWeight::Thin => "Thin",
        FontWeight::Light => "Light",
        FontWeight::Normal => "Normal",
        FontWeight::Medium => "Medium",
        FontWeight::Bold => "Bold",
    });
    size.set(cx, "lightFontWeight", a)?;

    configjs.set(cx, "font", font)?;

    Ok(configjs)
}
