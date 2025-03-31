import { theme } from "ant-design-vue";
import { defineStore } from "pinia";
import variables from "../styles/variables.module.scss";

export const useUIStore = defineStore("ui", {
  state: () => ({
    siderCollapsed: false,
    themeName: "red",
    darkMode: "dark",
  }),
  getters: {
    darkModeComp: (state) => state.darkMode,
    themeConfig: (state) => {
      document.documentElement.setAttribute("data-dark", state.darkMode);
      document.documentElement.setAttribute("data-theme", state.themeName);
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
        algorithm: state.darkMode === "light" ? theme.defaultAlgorithm : theme.darkAlgorithm,
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
    toggleDarkMode() {
      this.darkMode = this.darkMode === "light" ? "dark" : "light";
    },
  },
  persist: true,
});
