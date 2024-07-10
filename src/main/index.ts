import { app, BrowserWindow, dialog, globalShortcut, ipcMain, Menu, shell } from "electron";
import { attachTitlebarToWindow, setupTitlebar } from "custom-electron-titlebar/main";
import { electronApp, is, optimizer, platform } from "@electron-toolkit/utils";
import icon from "../../resources/icon.png?asset";
import { join } from "path";

setupTitlebar();

app.commandLine.appendSwitch("disable-features", "WidgetLayering");

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app
  .whenReady()
  .then(() => {
    // const backendProc = spawn(backend);
    // backendProc.stdout.on("data", (data) => {
    //   // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call
    //   console.log("data: ", data.toString("utf8"));
    // });
    // backendProc.stderr.on("data", (data) => {
    //   dialog
    //     .showMessageBox({
    //       type: "error",
    //       title: "Error",
    //       message: "Importing data failed.",
    //       // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    //       detail: data.toString("utf8"),
    //     })
    //     .catch(console.error);
    //   console.log(`stderr: ${data}`); // when error
    // });

    // Set app user model id for windows
    electronApp.setAppUserModelId("com.nc");
    // Default open or close DevTools by F12 in development
    // and ignore CommandOrControl + R in production.
    // see https://github.com/alex8088/electron-toolkit/tree/master/packages/utils
    app.on("browser-window-created", (_, window) => {
      optimizer.watchWindowShortcuts(window);
    });

    function createWindow() {
      // Create the browser window.
      const minHeight = 500;
      const minWidth = 320;

      const mainWindow = new BrowserWindow({
        width: minWidth,
        height: minHeight,
        minWidth,
        minHeight,
        show: false,
        titleBarStyle: "hidden",
        ...(process.platform === "linux" ? { icon } : {}),
        webPreferences: {
          preload: join(__dirname, "../preload/index.js"),
          devTools: is.dev,
          sandbox: false,
        },
      });
      Menu.setApplicationMenu(null);

      attachTitlebarToWindow(mainWindow);
      mainWindow.setMinimumSize(minWidth, minHeight);

      mainWindow.on("ready-to-show", () => {
        mainWindow.show();
      });

      mainWindow.webContents.setWindowOpenHandler((details) => {
        shell.openExternal(details.url).catch(console.error);
        return { action: "deny" };
      });

      // HMR for renderer base on electron-vite cli.
      // Load the remote URL for development or the local html file for production.
      if (is.dev && process.env["ELECTRON_RENDERER_URL"]) {
        mainWindow.loadURL(process.env["ELECTRON_RENDERER_URL"]).catch(console.error);
      }
      else {
        mainWindow.loadFile(join(__dirname, "../renderer/index.html")).catch(console.error);
      }
    }

    if (is.dev) {
      globalShortcut.register("CommandOrControl+Shift+I", () => {
        BrowserWindow.getFocusedWindow()?.webContents.toggleDevTools();
      });
    }

    createWindow();

    app.on("activate", () => {
      // On macOS it's common to re-create a window in the app when the
      // dock icon is clicked and there are no other windows open.
      if (BrowserWindow.getAllWindows().length === 0) {
        createWindow();
      }
    });

    // Quit when all windows are closed, except on macOS. There, it's common
    // for applications and their menu bar to stay active until the user quits
    // explicitly with Cmd + Q.
    app.on("window-all-closed", () => {
      if (process.platform !== "darwin") {
        // backendProc.kill();
        app.quit();
      }
    });
    app.on("before-quit", () => {
      // backendProc.kill();
    });
  })
  .catch(console.error);

// In this file you can include the rest of your app"s specific main process
// code. You can also put them in separate files and require them here.

// ipcMain.handle("importData", async () => {
//   const { canceled, filePaths } = await dialog.showOpenDialog({
//     properties: ["openFile"],
//     filters: [{ name: "BEAM Files", extensions: ["csv"] }],
//   });
//   if (!canceled) {
//     const response = await fetch("http://localhost:5000/api/input_csv", {
//       method: "POST",
//       headers: {
//         // eslint-disable-next-line @typescript-eslint/naming-convention
//         "Content-Type": "application/json",
//       },
//       redirect: "follow",
//       body: JSON.stringify({
//         // eslint-disable-next-line @typescript-eslint/naming-convention, camelcase
//         file_name: filePaths[0],
//       }),
//     });
//     // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
//     const data: Assembly[] | { [key: string]: string } = await response.json();
//     if (!Array.isArray(data)) {
//       if (Object.keys(data).includes("error")) {
//         await dialog.showMessageBox({
//           type: "error",
//           title: "Error",
//           message: "Importing data failed.",
//           detail: data["error"],
//         });
//         return [] as EditAssembly[];
//       }
//     }
//     else {
//       return data.map<EditAssembly>((val) => {
//         val["editing"] = false;
//         val["selected"] = false;
//         val["inputFileName"] = filePaths[0];
//         return val as EditAssembly;
//       });
//     }
//   }
//   return [] as EditAssembly[];
// });

// ipcMain.handle("writeData", async (_event, data: EditAssembly[]) => {
//   const assemblies: Assembly[] = data.map<Assembly>((val) => ({
//     id: val.id,
//     // eslint-disable-next-line camelcase, @typescript-eslint/naming-convention
//     assembly_no: val.assembly_no,
//     // eslint-disable-next-line @typescript-eslint/naming-convention, camelcase
//     assembly_name: val.assembly_name,
//     hns: val.hns,
//     mtos: val.mtos,
//     parts: val.parts,
//   }));
//   const inputFileName = data[0].inputFileName;

//   const { canceled, filePath } = await dialog.showSaveDialog({
//     filters: [{ name: "Parts List", extensions: ["xlsx"] }],
//   });
//   await new Promise((resolve) => {
//     if (!canceled) {
//       fetch("http://localhost:5000/api/write_assemblies", {
//         method: "POST",
//         headers: {
//           // eslint-disable-next-line @typescript-eslint/naming-convention
//           "Content-Type": "application/json",
//         },
//         redirect: "follow",
//         body: JSON.stringify({
//           // eslint-disable-next-line @typescript-eslint/naming-convention, camelcase
//           file_name: filePath,
//           assemblies,
//           // eslint-disable-next-line @typescript-eslint/naming-convention, camelcase
//           input_file_name: inputFileName,
//         }),
//       })
//         .then((writeResponse) => writeResponse.json())
//         .then(
//           async (writeResponse: { status: string; error: string }) => {
//             if (writeResponse.status === "success") {
//               await dialog.showMessageBox({
//                 type: "info",
//                 title: "Success",
//                 message: "Data written successfully.",
//               });
//               resolve("");
//             }
//             else {
//               await dialog.showMessageBox({
//                 type: "error",
//                 title: "Error",
//                 message: "Failed to write data.",
//                 detail: writeResponse.error,
//               });
//               resolve("");
//             }
//           },
//           async (err) => {
//             await dialog.showMessageBox({
//               type: "error",
//               title: "Error",
//               message: "Failed to get status of temp data.",
//               // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
//               detail: err.toString(),
//             });
//             resolve("");
//           },
//         );
//     }
//     else {
//       resolve("");
//     }
//   });
// });

// ipcMain.handle("previewData", async (_event, data: EditAssembly[]) => {
//   const assemblies: Assembly[] = data.map<Assembly>((val) => ({
//     id: val.id,
//     // eslint-disable-next-line camelcase, @typescript-eslint/naming-convention
//     assembly_no: val.assembly_no,
//     // eslint-disable-next-line @typescript-eslint/naming-convention, camelcase
//     assembly_name: val.assembly_name,
//     hns: val.hns,
//     mtos: val.mtos,
//     parts: val.parts,
//   }));
//   const inputFileName = data[0].inputFileName;

//   const parentWindow = BrowserWindow.getAllWindows()[0];
//   const previewWindow = new BrowserWindow({
//     parent: parentWindow,
//     modal: true,
//     title: "Preview",
//     width: 1080,
//     height: 720,
//     minWidth: 1080,
//     minHeight: 720,
//     show: true,
//     acceptFirstMouse: true,
//     titleBarStyle: "hidden",
//     ...(process.platform === "linux" ? { icon } : {}),
//     webPreferences: {
//       preload: join(__dirname, "../preload/index.js"),
//       devTools: is.dev,
//       sandbox: false,
//     },
//   });

//   return await new Promise((resolve) => {
//     fetch("http://localhost:5000/api/get_preview_data", {
//       method: "POST",
//       headers: {
//         // eslint-disable-next-line @typescript-eslint/naming-convention
//         "Content-Type": "application/json",
//       },
//       redirect: "follow",
//       body: JSON.stringify({
//         // eslint-disable-next-line @typescript-eslint/naming-convention, camelcase
//         file_name: "",
//         assemblies,
//         // eslint-disable-next-line @typescript-eslint/naming-convention, camelcase
//         input_file_name: inputFileName,
//       }),
//     })
//       .then((res) => res.json())
//       .then(
//         async (res: { buffer: number[] } | { status: string; error: string }) => {
//           if (Object.keys(res).includes("error")) {
//             await dialog.showMessageBox({
//               type: "error",
//               title: "Error",
//               message: "Failed to write temp data.",
//               detail: (res as { status: string; error: string }).error,
//             });
//             resolve("");
//           }
//           else {
//             if (is.dev && process.env["ELECTRON_RENDERER_URL"]) {
//               await previewWindow.loadURL(process.env["ELECTRON_RENDERER_URL"] + "/preview.html");
//               previewWindow.webContents.send("previewData", (res as { buffer: number[] }).buffer);
//             }
//             else {
//               await previewWindow.loadFile(join(__dirname, "../renderer/preview.html"));
//             }
//             previewWindow.webContents.send("previewData", (res as { buffer: number[] }).buffer);

//             previewWindow.on("ready-to-show", () => {
//               // Show: false by default, then show when ready to prevent page "flicker"
//               console.log("here");

//               previewWindow.show();
//             });
//             previewWindow.on("closed", () => {
//               resolve("");
//             });
//           }
//         },
//         async (err) => {
//           await dialog.showMessageBox({
//             type: "error",
//             title: "Error",
//             message: "Failed to get status of temp data.",
//             // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
//             detail: err.toString(),
//           });
//           resolve("");
//         },
//       );
//   });
// });

ipcMain.handle("showErrorBox", async (_event, message: string, detail: string) => {
  await dialog.showMessageBox({
    type: "error",
    title: "Error",
    message,
    detail,
  });
});
