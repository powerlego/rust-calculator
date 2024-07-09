/* eslint-disable @typescript-eslint/no-explicit-any */
import { ElectronAPI } from "@electron-toolkit/preload";
declare global {
  interface Window {
    electron: ElectronAPI;
    api: {
      showErrorBox: (message: string, detail: string) => Promise<void>;
      on: (channel: string, listener: (...args: any[]) => void) => Electron.IpcRenderer;
      off: (channel: string, listener: (...args: any[]) => void) => Electron.IpcRenderer;
    };
  }
}
