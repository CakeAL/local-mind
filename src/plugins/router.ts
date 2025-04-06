import router from "@/router";
import { App } from "vue";

export function setupRouter(app: App) {
  app.use(router);
}
