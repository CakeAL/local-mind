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

onMounted(() => {
  getModelList();
});

const getModelList = async () => {
  let res = await invoke<ModelList>("list_models").catch(
    (err) => message.warning(err),
  );
  chatStore.setModelList(res);
};

const newAssistant = async () => {
  let res = await invoke<AssistantInfo>("new_assistant").catch(
    (err) => message.warning(err),
  );
  if (res.uuid) {
    assistants.value.push(res);
  }
};
</script>
<template>
  <a-space direction="vertical" class="list">
    <SideItem
      v-for="(assistant, index) in assistants"
      :key="index"
      :title="assistant.name"
      :showSettingIcon="true"
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
    <p>Some contents...</p>
    <p>Some contents...</p>
    <p>Some contents...</p>
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
</style>
