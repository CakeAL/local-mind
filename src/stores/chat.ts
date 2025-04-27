import { AssistantInfo } from "@/models/assistant";
import { defineStore } from "pinia";

export const useChatStore = defineStore("chat", {
  state: () => ({
    curAssistant: null as AssistantInfo | null,
  }),
  getters: {
    getCurAssistant: (state) => state.curAssistant,
  },
  actions: {
    setCurAssistant(assistant: AssistantInfo) {
      this.curAssistant = assistant;
    },
    clear() {
      this.curAssistant = null;
    },
  },
});
