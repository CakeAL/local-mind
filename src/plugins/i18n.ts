import { loadLocalemessages } from "@/locales";
import type { App } from "vue";
import { createI18n } from "vue-i18n";

const locale = getLocale();

function getLocale(): string {
  const uiStr = localStorage.getItem("ui");
  let locale = "";
  if (uiStr) {
    try {
      const ui = JSON.parse(uiStr);
      locale = ui.locale;
    } catch (e) {
      console.log(e);
      locale = "zh_CN";
    }
  }
  return locale;
}

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
