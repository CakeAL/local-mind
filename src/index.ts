import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";
import { setupI18n } from "./plugins/i18n";
import { setupPinia } from "./plugins/pinia";
import router from "./router";

const app = createApp(App);

setupPinia(app);
await setupI18n(app);
app.use(router);

app.mount("#root");

// 阻止浏览器默认行为
document.addEventListener("drop", function(e) {
  e.preventDefault();
}, false);
document.addEventListener("dragover", function(e) {
  e.preventDefault();
}, false);
