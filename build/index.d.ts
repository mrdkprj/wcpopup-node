import { Config, Menu, MenuItem, MenuItemConstructorOptions, Theme } from "../lib";

declare namespace PopupMenu {

    type PopupMenuItem = {
        id: string;
        type: "normal" | "separator" | "submenu" | "checkbox" | "radio";
        label: string;
        accelerator: string;
        enabled: boolean;
        checked: boolean;
        submenu: Menu;
        name: string;
        readonly uuid:number;
    };

    function getDefaultConfig(): Config;
    function buildFromTemplate(menuWindowHandle:number, template:MenuItemConstructorOptions[]): number;
    function buildFromTemplateWithTheme(menuWindowHandle:number, template:MenuItemConstructorOptions[], theme:Theme): number;
    function buildFromTemplateWithConfig(menuWindowHandle:number, template:MenuItemConstructorOptions[], config:Config): number;
    function popup(menuWindowHandle:number, x:number, y:number): Promise<PopupMenuItem>;
    function items(menuWindowHandle:number): PopupMenuItem[];
    function remove(menuWindowHandle:number, item:MenuItem): void;
    function removeAt(menuWindowHandle:number, index:number): void;
    function append(menuWindowHandle:number, item:MenuItem): void;
    function insert(menuWindowHandle:number, index:number, item:MenuItem): void;
    function setTheme(menuWindowHandle:number, theme:Theme): void;
    function getMenuItemById(menuWindowHandle:number, id:string): PopupMenuItem | void;
}

export = PopupMenu;