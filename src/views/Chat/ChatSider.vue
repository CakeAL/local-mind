<script setup lang="ts">
import SideItem from "@/components/SideItem.vue";
import { AssistantInfo } from "@/models/assistant";
import { ModelList } from "@/models/model";
import { useChatStore } from "@/stores/chat";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import { onMounted, ref } from "vue";

const assistants = ref<AssistantInfo[]>([]);
const openModal = ref(false);
const chatStore = useChatStore();
const selectedModelName = ref<String>("");

onMounted(() => {
  getModelList();
  getAllAssistant().then(() => {
    if (assistants.value.length !== 0) {
      chatStore.setCurAssistant(assistants.value[0]);
    }
  });
});

const getAllAssistant = async () => {
  let res = await invoke<AssistantInfo[]>("get_all_assistant").catch(
    (err) => message.warning(err),
  );
  if (res.length !== 0) {
    assistants.value = res;
  }
};

const getModelList = async () => {
  if (chatStore.modelListIsEmpty) {
    let res = await invoke<ModelList>("list_models").catch(
      (err) => message.warning(err),
    );
    chatStore.setModelList(res);
  }
};

const newAssistant = async () => {
  openModal.value = !openModal.value;
  if (!selectedModelName.value) {
    return;
  }
  let res = await invoke<AssistantInfo>("new_assistant", {
    model: selectedModelName.value,
  }).catch(
    (err) => message.warning(err),
  );
  if (res.uuid) {
    assistants.value.push(res);
  }
};

const selectThisAssistant = (assistant: AssistantInfo) => {
  chatStore.setCurAssistant(assistant);
};
</script>
<template>
  <a-space direction="vertical" class="list">
    <SideItem
      v-for="(assistant, index) in assistants"
      :key="index"
      :title="assistant.name"
      :showSettingIcon="true"
      :callback="() => selectThisAssistant(assistant)"
      :selected="assistant.uuid === chatStore.curAssistant?.uuid"
    />
    <a-divider style="margin: 0" v-if="assistants.length !== 0" />
    <SideItem
      :title="$t('chat.new-assistant')"
      :callback="() => openModal = !openModal"
    />
  </a-space>
  <a-modal
    v-model:open="openModal"
    :title="$t('chat.select-model')"
    @ok="newAssistant"
  >
    <p v-if="chatStore.modelListIsEmpty">
      {{ $t("chat.no-models") }}
    </p>
    <a-radio-group
      v-else
      v-model:value="selectedModelName"
      style="width: 100%"
      button-style="solid"
    >
      <a-radio-button
        v-for="(model, index) in chatStore.getModels"
        :key="index"
        :value="model.name"
        class="radio-button"
      >{{ model.name }}</a-radio-button>
    </a-radio-group>
  </a-modal>
</template>
<style scoped>
.list {
  user-select: none;
  width: 100%;
  height: 100%;
  overflow-y: scroll;
  padding: 10px;
}

.radio-button {
  width: 100%;
  display: flex;
  border-radius: 5px;
  border: none;
  margin-bottom: 5px;
  box-sizing: border-box;
}

.radio-button::before {
  width: 0;
}
</style>
