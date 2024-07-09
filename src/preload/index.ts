import { contextBridge, ipcRenderer } from "electron";
import { Titlebar, TitlebarColor } from "custom-electron-titlebar";
import { electronAPI } from "@electron-toolkit/preload";

window.addEventListener("DOMContentLoaded", () => {
  new Titlebar({
    backgroundColor: TitlebarColor.BLACK,
    containerOverflow: "hidden",
    titleHorizontalAlignment: "center",
  });
});

// Custom APIs for renderer
const api = {
  showErrorBox: (message: string, detail: string) => ipcRenderer.invoke("showErrorBox", message, detail),
  on: (channel: string, listener: (...args: any[]) => void) => ipcRenderer.on(channel, listener),
  off: (channel: string, listener: (...args: any[]) => void) => ipcRenderer.off(channel, listener),
};

// Use `contextBridge` APIs to expose Electron APIs to
// renderer only if context isolation is enabled, otherwise
// just add to the DOM global.
if (process.contextIsolated) {
  try {
    contextBridge.exposeInMainWorld("electron", electronAPI);
    contextBridge.exposeInMainWorld("api", api);
  }
  catch (error) {
    console.error(error);
  }
}
else {
  // @ts-ignore (define in dts)
  window.electron = electronAPI;
  // @ts-ignore (define in dts)
  window.api = api;
}
