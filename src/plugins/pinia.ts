import { useAppStore } from "@/stores/app";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import type { App } from "vue";

export function setupPinia(app: App) {
  const pinia = createPinia();
  pinia.use(piniaPluginPersistedstate);
  app.use(pinia);

  const appStore = useAppStore();
  appStore.refresh();
  console.log(appStore.getModels);
}
