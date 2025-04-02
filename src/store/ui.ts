import { theme } from "ant-design-vue";
import enUS from "ant-design-vue/es/locale/en_US";
import zhCN from "ant-design-vue/es/locale/zh_CN";
import { defineStore } from "pinia";
import variables from "../styles/variables.module.scss";

export const themeMedia = window.matchMedia("(prefers-color-scheme: dark)");

export const useUIStore = defineStore("ui", {
  state: () => ({
    siderCollapsed: false,
    themeName: "green",
    darkMode: "auto",
    themeReset: true,
    locale: "zh_CN",
  }),
  getters: {
    getDarkMode: (state) => state.darkMode,
    getLocale: (state) => {
      switch (state.locale) {
        case "zh_CN":
          return zhCN;
        case "en_US":
          return enUS;
        default:
          return enUS;
      }
    },
    getThemeConfig: (state) => {
      state.themeReset = !state.themeReset;
      document.documentElement.setAttribute("data-dark", state.darkMode);
      document.documentElement.setAttribute("data-theme", state.themeName);
      let algorithm = theme.defaultAlgorithm;
      if (state.darkMode === "auto" && themeMedia.matches || state.darkMode === "dark") {
        algorithm = theme.darkAlgorithm;
      }
      // 主题配置
      return {
        token: {
          colorPrimary: variables[state.themeName] || "#27ba9b",
          colorSuccess: "#1dc779",
          colorWarning: "#ffb302",
          colorError: "#cf4444",
          colorInfo: variables[state.themeName] || "#27ba9b",
          wireframe: true,
        },
        algorithm: algorithm,
      };
    },
  },
  actions: {
    toggleSiderCollapsed() {
      this.siderCollapsed = !this.siderCollapsed;
    },
    setThemeName(value: string) {
      this.themeName = value;
    },
    toggleDarkMode(value: string) {
      this.darkMode = value;
    },
    toggleThemeReset() {
      this.themeReset = !this.themeReset;
    },
  },
  persist: true,
});
