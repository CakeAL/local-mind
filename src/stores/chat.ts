import { ModelList } from "@/models/model";
import { defineStore } from "pinia";

export const useChatStore = defineStore("chat", {
  state: () => ({
    modelList: {
      models: [],
    } as ModelList,
  }),
  getters: {
    getModelList: (state) => state.modelList,
  },
  actions: {
    setModelList(modelList: ModelList) {
      this.modelList = modelList;
    },
  },
});
