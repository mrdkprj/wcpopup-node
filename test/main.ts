import { app, BrowserWindow, ipcMain, nativeTheme } from "electron";
import os from "os";
import path from "path";
import { getDefaultConfig, Menu, MenuItem, MenuItemConstructorOptions } from "../lib/index";

let menu: Menu;
let dark = true;
const createWindow = () => {
    nativeTheme.themeSource = "dark";
    const win = new BrowserWindow({
        title: "main",
        width: 800,
        height: 601,
        // darkTheme:true,

        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win.loadFile("index.html");

    menu = new Menu();
    const hbuf = win.getNativeWindowHandle();
    let hwnd = 0;

    if (os.platform() == "linux") {
        if (os.endianness() == "LE") {
            hwnd = hbuf.readUInt32LE();
        } else {
            hwnd = hbuf.readUInt32BE();
        }
    } else {
        if (os.endianness() == "LE") {
            hwnd = hbuf.readInt32LE();
        } else {
            hwnd = hbuf.readInt32BE();
        }
    }
    let config = getDefaultConfig();
    console.log(config.icon);
    config.theme = "dark";
    config.size.itemVerticalPadding = 10;
    menu.buildFromTemplateWithConfig(hwnd, getTemp(), config);

    const win2 = new BrowserWindow({
        title: "sub",
        parent: win,
        width: 800,
        height: 601,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win2.loadFile("index2.html");
    const menu2 = new Menu();
    const hbuf2 = win2.getNativeWindowHandle();

    if (os.endianness() == "LE") {
        hwnd = hbuf2.readUInt32LE();
    } else {
        hwnd = hbuf2.readUInt32BE();
    }

    menu2.buildFromTemplateWithConfig(hwnd, getTemp(), config);
};

const handleSetTitle = async (_event: any, pos: any) => {
    console.log("start");
    await menu.popup(pos.x, pos.y);
};

const toggle = () => {
    dark = !dark;
    nativeTheme.themeSource = dark ? "dark" : "light";
    menu.setTheme(nativeTheme.themeSource);
};

let apflg = false;
const append = () => {
    apflg = !apflg;

    if (apflg) {
        const submenu = menu.getMenuItemById("theme");
        if (submenu) {
            submenu.submenu?.append({
                id: "test1234",
                accelerator: "F2",
                label: "Test for sub",
                click: callback,
            });
        }
    } else {
        const todel = menu.getMenuItemById("test1234");
        console.log(todel);
        const submenu = menu.getMenuItemById("theme");
        if (submenu && todel) {
            submenu.submenu?.remove(todel);
        }
    }
};

const reload = () => {
    const submenu = menu.getMenuItemById("theme");
    if (submenu) {
        let items = submenu.submenu?.items();
        console.log(items);
    }
};

app.whenReady().then(async () => {
    createWindow();
    ipcMain.on("set-title", handleSetTitle);
    ipcMain.on("toggle", toggle);
    ipcMain.on("append", append);
    ipcMain.on("reload", reload);
});

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") app.quit();
});

const callback = (a: MenuItem) => {
    console.log(a);
};

const getTemp = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "playbackSpeed",
            label: t("playbackSpeed"),
            submenu: playbackSpeedMenu(),
        },
        {
            id: "seekSpeed",
            label: t("seekSpeed"),
            submenu: seekSpeedMenu(),
        },
        {
            id: "fitToWindow",
            label: t("fitToWindow"),
            type: "checkbox",
            checked: false,
            click: callback,
        },
        { type: "separator" },
        {
            id: "playbackSpeed",
            //label: t("playlist"),
            label: t("playbackSpeed"),
            //accelerator: "CmdOrCtrl+P",

            click: callback,
        },
        {
            id: "fullscreen",
            label: t("fullscreen"),
            accelerator: "F11",
            click: callback,
        },
        {
            id: "pip",
            label: t("pip"),
            click: callback,
        },
        { type: "separator" },
        {
            id: "capture",
            label: t("capture"),
            //accelerator: "CmdOrCtrl+S",
            click: callback,
        },
        { type: "separator" },
        {
            id: "theme",
            label: t("theme"),
            submenu: themeMenu(),
        },
    ];

    return template;
};

const themeMenu = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "light",
            label: t("light"),
            type: "checkbox",
            checked: false,
            click: callback,
        },
        {
            id: "dark",
            label: t("dark"),
            type: "checkbox",
            checked: true,
            click: callback,
        },
    ];

    return template;
};

const playbackSpeedMenu = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "playbackrate0",
            label: "0.25",
            type: "radio",
            name: "seekSpeed",
            click: callback,
        },
        {
            id: "playbackrate1",
            label: "0.5",
            type: "radio",
            name: "seekSpeed",
            click: callback,
        },
        {
            id: "playbackrate2",
            label: "0.75",
            type: "radio",
            name: "seekSpeed",
            click: callback,
        },
        {
            id: "playbackrate3",
            label: `1 - ${t("default")}`,
            type: "radio",
            name: "seekSpeed",
            click: callback,
            checked: true,
        },
        {
            id: "playbackrate4",
            label: "1.25",
            type: "radio",
            name: "seekSpeed",
            click: callback,
        },
        {
            id: "playbackrate5",
            label: "1.5",
            type: "radio",
            name: "seekSpeed",
            click: callback,
        },
        {
            id: "playbackrate6",
            label: "1.75",
            type: "radio",
            name: "seekSpeed",
            click: callback,
        },
        {
            id: "playbackrate7",
            label: "2",
            type: "radio",
            name: "seekSpeed",
            click: callback,
        },
    ];

    return template;
};

const seekSpeedMenu = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "seekspeed0",
            label: `0.03${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed1",
            label: `0.05${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed2",
            label: `0.1${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed3",
            label: `0.5${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed4",
            label: `1${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed5",
            label: `3${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed6",
            label: `5${t("second")} - ${t("default")}`,
            type: `radio`,
            checked: true,
            name: "seekSpeed",
        },
        {
            id: "seekspeed7",
            label: `10${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed8",
            label: `20${t("second")}`,
            type: "radio",
            name: "seekSpeed",
        },
    ];

    return template;
};

const t = (a: string) => a;
