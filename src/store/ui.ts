import { defineStore } from "pinia";

export const useUIStore = defineStore("ui", {
  state: () => ({
    siderCollapsed: false,
  }),
  actions: {
    toggleSiderCollapsed() {
      this.siderCollapsed = !this.siderCollapsed;
    },
  },
});
