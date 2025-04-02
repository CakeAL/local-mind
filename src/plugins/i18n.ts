import type { App } from "vue";
import { createI18n } from "vue-i18n";

// 示例语言包
const messages = {
  en: { message: { hello: "Hello" } },
  zh_CN: { message: { hello: "你好" } },
};

export function setupI18n(app: App) {
  const i18n = createI18n({
    legacy: false, 
    locale: "zh_CN", // 默认语言
    fallbackLocale: "en", // 备用语言
    messages,
  });

  app.use(i18n);
}
