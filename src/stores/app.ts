import { ModelList } from "@/models/model";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import { defineStore } from "pinia";

export const useAppStore = defineStore("app", {
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
    async refresh() {
      let res = await invoke<ModelList>("list_models").catch(
        (err) => message.warning(err),
      );
      this.modelList = res;
    },
  },
});
