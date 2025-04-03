import type { App } from "vue";
import { createI18n } from "vue-i18n";
import { loadLocalemessages } from "../locales";

const locale = localStorage.getItem("locale") || "zh_CN";

export const i18n = createI18n({
  legacy: false,
  locale,
  fallbackLocale: "en",
  messages: {},
});

export async function setupI18n(app: App) {
  const messages = await loadLocalemessages(locale);
  i18n.global.setLocaleMessage(locale, messages);
  app.use(i18n);
}
