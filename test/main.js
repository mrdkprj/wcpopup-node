"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var electron_1 = require("electron");
var os_1 = __importDefault(require("os"));
var path_1 = __importDefault(require("path"));
var index_1 = require("../lib/index");
var menu;
var dark = true;
var createWindow = function () {
    electron_1.nativeTheme.themeSource = "dark";
    var win = new electron_1.BrowserWindow({
        title: "main",
        width: 800,
        height: 601,
        // darkTheme:true,
        webPreferences: {
            preload: path_1.default.join(__dirname, "preload.js"),
        },
    });
    win.loadFile("index.html");
    menu = new index_1.Menu();
    var hbuf = win.getNativeWindowHandle();
    var hwnd = 0;
    if (os_1.default.platform() == "linux") {
        if (os_1.default.endianness() == "LE") {
            hwnd = hbuf.readUInt32LE();
        }
        else {
            hwnd = hbuf.readUInt32BE();
        }
    }
    else {
        if (os_1.default.endianness() == "LE") {
            hwnd = hbuf.readInt32LE();
        }
        else {
            hwnd = hbuf.readInt32BE();
        }
    }
    var config = (0, index_1.getDefaultConfig)();
    console.log(config.icon);
    config.theme = "dark";
    config.size.itemVerticalPadding = 10;
    menu.buildFromTemplateWithConfig(hwnd, getTemp(), config);
    var win2 = new electron_1.BrowserWindow({
        title: "sub",
        parent: win,
        width: 800,
        height: 601,
        webPreferences: {
            preload: path_1.default.join(__dirname, "preload.js"),
        },
    });
    win2.loadFile("index2.html");
    var menu2 = new index_1.Menu();
    var hbuf2 = win2.getNativeWindowHandle();
    if (os_1.default.endianness() == "LE") {
        hwnd = hbuf2.readUInt32LE();
    }
    else {
        hwnd = hbuf2.readUInt32BE();
    }
    menu2.buildFromTemplateWithConfig(hwnd, getTemp(), config);
};
var handleSetTitle = function (_event, pos) { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                console.log("start");
                return [4 /*yield*/, menu.popup(pos.x, pos.y)];
            case 1:
                _a.sent();
                return [2 /*return*/];
        }
    });
}); };
var toggle = function () {
    dark = !dark;
    electron_1.nativeTheme.themeSource = dark ? "dark" : "light";
    menu.setTheme(electron_1.nativeTheme.themeSource);
};
var apflg = false;
var append = function () {
    var _a, _b;
    apflg = !apflg;
    if (apflg) {
        var submenu = menu.getMenuItemById("theme");
        if (submenu) {
            (_a = submenu.submenu) === null || _a === void 0 ? void 0 : _a.append({
                id: "test1234",
                accelerator: "F2",
                label: "Test for sub",
                click: callback,
            });
        }
    }
    else {
        var todel = menu.getMenuItemById("test1234");
        console.log(todel);
        var submenu = menu.getMenuItemById("theme");
        if (submenu && todel) {
            (_b = submenu.submenu) === null || _b === void 0 ? void 0 : _b.remove(todel);
        }
    }
};
var reload = function () {
    var _a;
    var submenu = menu.getMenuItemById("theme");
    if (submenu) {
        var items = (_a = submenu.submenu) === null || _a === void 0 ? void 0 : _a.items();
        console.log(items);
    }
};
electron_1.app.whenReady().then(function () { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        createWindow();
        electron_1.ipcMain.on("set-title", handleSetTitle);
        electron_1.ipcMain.on("toggle", toggle);
        electron_1.ipcMain.on("append", append);
        electron_1.ipcMain.on("reload", reload);
        return [2 /*return*/];
    });
}); });
electron_1.app.on("window-all-closed", function () {
    if (process.platform !== "darwin")
        electron_1.app.quit();
});
var callback = function (a) {
    console.log(a);
};
var getTemp = function () {
    var template = [
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
var themeMenu = function () {
    var template = [
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
var playbackSpeedMenu = function () {
    var template = [
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
            label: "1 - ".concat(t("default")),
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
var seekSpeedMenu = function () {
    var template = [
        {
            id: "seekspeed0",
            label: "0.03".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed1",
            label: "0.05".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed2",
            label: "0.1".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed3",
            label: "0.5".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed4",
            label: "1".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed5",
            label: "3".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed6",
            label: "5".concat(t("second"), " - ").concat(t("default")),
            type: "radio",
            checked: true,
            name: "seekSpeed",
        },
        {
            id: "seekspeed7",
            label: "10".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
        {
            id: "seekspeed8",
            label: "20".concat(t("second")),
            type: "radio",
            name: "seekSpeed",
        },
    ];
    return template;
};
var t = function (a) { return a; };
