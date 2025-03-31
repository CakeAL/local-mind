import { defineConfig, loadEnv } from "@rsbuild/core";
import { pluginSass } from "@rsbuild/plugin-sass";
import { pluginVue } from "@rsbuild/plugin-vue";
import { AntDesignVueResolver } from "unplugin-vue-components/resolvers";
import Components from "unplugin-vue-components/rspack";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [
    pluginVue(),
    pluginSass(),
  ],
  tools: {
    rspack: {
      plugins: [
        Components({
          resolvers: [
            AntDesignVueResolver({
              importStyle: false, // css in js
            }),
          ],
        }),
      ],
    },
  },
  performance: {
    chunkSplit: {
      strategy: "split-by-experience",
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
