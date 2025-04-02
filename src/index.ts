import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";
import { setupI18n } from "./plugins/i18n";
import { setupPinia } from "./plugins/pinia";

const app = createApp(App);

setupPinia(app);
setupI18n(app);

app.mount("#root");
