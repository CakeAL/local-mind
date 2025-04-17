import { ModelList } from "@/models/model";
import { defineStore } from "pinia";

export const useChatStore = defineStore("chat", {
  state: () => ({
    modelList: {
      models: [],
    } as ModelList,
  }),
  getters: {
    getModels: (state) => state.modelList.models,
    modelListIsEmpty: (state) => state.modelList.models.length === 0,
  },
  actions: {
    setModelList(modelList: ModelList) {
      this.modelList = modelList;
    },
  },
});
