import { defineConfig, externalizeDepsPlugin } from "electron-vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath, URL } from "node:url";
import { resolve } from "path";

export default defineConfig({
  main: {
    plugins: [externalizeDepsPlugin()],
    build: {
      rollupOptions: {
        input: {
          index: resolve(__dirname, "src/main/index.ts"),
        },
      },
      minify: true,
    },
  },
  preload: {
    plugins: [externalizeDepsPlugin()],
    build: {
      minify: true,
    },
  },
  renderer: {
    plugins: [svelte()],
    resolve: {
      alias: {
        "@mescius/spread-sheets": fileURLToPath(new URL("./node_modules/@mescius/spread-common", import.meta.url)),
      },
    },
    build: {
      rollupOptions: {
        input: {
          index: resolve(__dirname, "src/renderer/index.html"),
          preview: resolve(__dirname, "src/renderer/preview.html"),
        },
      },
      minify: true,
    },
  },
});
