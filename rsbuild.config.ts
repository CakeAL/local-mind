import { defineConfig, loadEnv } from "@rsbuild/core";
import { pluginVue } from "@rsbuild/plugin-vue";

const { publicVars } = loadEnv({ prefixes: ["VITE_"] });
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [
    pluginVue(),
  ],
  performance: {
    chunkSplit: {
      strategy: "split-by-experience",
    },
  },
  html: {
    template: "./index.html",
  },
  source: {
    define: publicVars,
    entry: {
      index: "./src/main.ts",
    },
  },
  dev: {
    watchFiles: {
      paths: "!src-tauri/**",
    },
    client: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
  },
  server: {
    port: 1420,
    strictPort: true,
    host: host || undefined,
  },
});
