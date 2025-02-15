import * as PopupMenu from "../build/index";

export type MenuItemType = "normal" | "separator" | "submenu" | "checkbox" | "radio";

export type MenuItemConstructorOptions = {
    id?: string;
    type?: MenuItemType;
    label?: string;
    accelerator?: string;
    enabled?: boolean;
    checked?: boolean;
    submenu?: MenuItemConstructorOptions[] | Menu;
    name?: string;
    click?: Function;
    icon?: string;
};

export type MenuItem = {
    id?: string;
    type?: MenuItemType;
    label?: string;
    accelerator?: string;
    enabled?: boolean;
    checked?: boolean;
    submenu?: Menu;
    name?: string;
    click?: Function;
};

export type Theme = "dark" | "light" | "system";
export type MenuSize = {
    borderSize: number;
    verticalMargin: number;
    horizontalMargin: number;
    itemVerticalPadding: number;
    itemHorizontalPadding: number;
    fontSize?: number;
    fontWeight?: number;
};

export type ColorScheme = {
    color: number;
    border: number;
    accelerator: number;
    disabled: number;
    backgroundColor: number;
    hoverBackgroundColor: number;
};

export type ThemeColor = {
    dark: ColorScheme;
    light: ColorScheme;
};

export type Corner = "Round" | "DoNotRound";

export type FontWeight = "Thin" | "Light" | "Normal" | "Medium" | "Bold";
export type MenuFont = {
    fontFamily: string;
    darkFontSize: number;
    darkFontWeight: FontWeight;
    lightFontSize: number;
    lightFontWeight: FontWeight;
};

export type Config = {
    theme: Theme;
    size: MenuSize;
    color: ThemeColor;
    corner: Corner;
    font: MenuFont;
};

export type MenuType = "main" | "submenu";

const UUID = "MenuItem";

export const getDefaultConfig = () => {
    return PopupMenu.getDefaultConfig();
};

export class Menu {
    private menuWindowHandle = 0;
    type = "";
    private callbacks: { [key: string]: Function } = {};
    private uuid = 0;

    private ready() {
        if (!this.menuWindowHandle) throw new Error("Menu does not exist");
    }

    getWindowHandle(): number {
        return this.menuWindowHandle;
    }

    buildFromTemplate(menuWindowHandle: number, template: MenuItemConstructorOptions[]) {
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.menuWindowHandle = PopupMenu.buildFromTemplate(menuWindowHandle, effectiveTemplate);
    }

    buildFromTemplateWithTheme(menuWindowHandle: number, template: MenuItemConstructorOptions[], theme: Theme) {
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.menuWindowHandle = PopupMenu.buildFromTemplateWithTheme(menuWindowHandle, effectiveTemplate, theme);
    }

    buildFromTemplateWithConfig(menuWindowHandle: number, template: MenuItemConstructorOptions[], config: Config) {
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.menuWindowHandle = PopupMenu.buildFromTemplateWithConfig(menuWindowHandle, effectiveTemplate, config);
    }

    private toEffectiveTemplates(items: MenuItemConstructorOptions[]): MenuItemConstructorOptions[] {
        return items.map((item) => {
            const newItem = this.toEffectiveTemplate(item);
            if (newItem.type == "submenu" && newItem.submenu) {
                this.toEffectiveTemplates(newItem.submenu as MenuItemConstructorOptions[]);
            }
            return newItem;
        });
    }

    private toEffectiveTemplate(item: MenuItemConstructorOptions | MenuItem): MenuItemConstructorOptions | MenuItem {
        item.id = this.getId(item.id);

        item.type = this.getType(item.type, item.submenu);

        if (item.click) {
            this.callbacks[item.id] = item.click;
        } else {
            this.callbacks[item.id] = () => {};
        }

        return item;
    }

    private getId(id: string | undefined): string {
        if (!id) {
            this.uuid++;
            return UUID + this.uuid;
        }

        return id;
    }

    private getType(type: MenuItemType | undefined, submenu: MenuItemConstructorOptions[] | Menu | undefined): MenuItemType {
        if (!type) {
            return submenu ? "submenu" : "normal";
        }

        return type;
    }

    private toMenuItem(item: PopupMenu.PopupMenuItem): MenuItem {
        const submenu = new Menu();
        if (item.submenu && Object.keys(item.submenu).length) {
            submenu.menuWindowHandle = item.submenu.menuWindowHandle;
            submenu.type = item.submenu.type;
        }
        return {
            ...item,
            click: this.callbacks[item.id],
            submenu,
        };
    }

    async popup(x: number, y: number) {
        this.ready();
        const result = await PopupMenu.popup(this.menuWindowHandle, x, y);
        if (Object.keys(result).length) {
            this.callbacks[result.id](result);
        }
    }

    items(): MenuItem[] {
        this.ready();
        return PopupMenu.items(this.menuWindowHandle).map((item) => this.toMenuItem(item));
    }

    remove(item: MenuItem) {
        this.ready();
        PopupMenu.remove(this.menuWindowHandle, item);
    }

    removeAt(index: number) {
        this.ready();
        PopupMenu.removeAt(this.menuWindowHandle, index);
    }

    append(item: MenuItem) {
        this.ready();
        PopupMenu.append(this.menuWindowHandle, this.toEffectiveTemplate(item) as MenuItem);
    }

    insert(index: number, item: MenuItem) {
        this.ready();
        PopupMenu.insert(this.menuWindowHandle, index, this.toEffectiveTemplate(item) as MenuItem);
    }

    setTheme(theme: Theme) {
        this.ready();
        PopupMenu.setTheme(this.menuWindowHandle, theme);
    }

    getMenuItemById(id: string): MenuItem | void {
        this.ready();
        const item = PopupMenu.getMenuItemById(this.menuWindowHandle, id);
        if (item) {
            return this.toMenuItem(item);
        }
        return item;
    }
}
