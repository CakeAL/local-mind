import { AssistantInfo } from "@/models/assistant";
import { ModelList } from "@/models/model";
import { defineStore } from "pinia";

export const useChatStore = defineStore("chat", {
  state: () => ({
    modelList: {
      models: [],
    } as ModelList,
    curAssistantUuid: "",
  }),
  getters: {
    getModels: (state) => state.modelList.models,
    modelListIsEmpty: (state) => state.modelList.models.length === 0,
    getCurAssistantUuid: (state) => state.curAssistantUuid,
  },
  actions: {
    setModelList(modelList: ModelList) {
      this.modelList = modelList;
    },
    setCurAssistant(assistant: AssistantInfo) {
      this.curAssistantUuid = assistant.uuid;
    },
  },
});
