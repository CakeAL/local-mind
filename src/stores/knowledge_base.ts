import { KnowledgeBaseInfo } from "@/models/knowledge_base";
import { defineStore } from "pinia";

export const useKnowledgeBaseStore = defineStore("KnowledgeBase", {
  state: () => ({
    curKnowledgeBase: null as KnowledgeBaseInfo | null,
  }),
  getters: {
    getCurKnowledgeBase: (state) => state.curKnowledgeBase,
  },
  actions: {
    setCurKnowledgeBase(knowledgeBaseInfo: KnowledgeBaseInfo) {
      this.curKnowledgeBase = knowledgeBaseInfo;
    },
    clear() {
      this.curKnowledgeBase = null;
    },
  },
});
