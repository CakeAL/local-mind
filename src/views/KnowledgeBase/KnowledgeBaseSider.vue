<script setup lang="ts">
import { KnowledgeBaseInfo } from "@/models/knowledge_base";
import { useAppStore } from "@/stores/app";
import { useKnowledgeBaseStore } from "@/stores/knowledge_base";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import { computed, onMounted, ref } from "vue";

const knowledgeBases = ref<KnowledgeBaseInfo[]>([]);
const knowledgeBaseStore = useKnowledgeBaseStore();
const appStore = useAppStore();
const openModal = ref(false);
const selectedModelName = ref<string>("");
const newKnowledgeBaseName = ref<string>("");

onMounted(() => {
  getAllKnowledgeBase().then(() => {
    if (knowledgeBases.value.length !== 0) {
      knowledgeBaseStore.setCurKnowledgeBase(knowledgeBases.value[0]);
    }
  });
});

const getAllKnowledgeBase = async () => {
  let res = await invoke<KnowledgeBaseInfo[]>("get_all_knowledge_base").catch((
    err,
  ) => message.warning(err));
  if (res.length !== 0) {
    knowledgeBases.value = res;
  }
};

const selectThisKnowledgeBase = (knowledgeBaseInfo: KnowledgeBaseInfo) => {
  knowledgeBaseStore.setCurKnowledgeBase(knowledgeBaseInfo);
};

const newKnowledgeBase = async () => {
  openModal.value = !openModal.value;
  if (!selectedModelName.value || !newKnowledgeBaseName.value) {
    message.warning("Please select a model and enter a name");
    return;
  }
  let res = await invoke<KnowledgeBaseInfo>("new_knowledge_base", {
    name: newKnowledgeBaseName.value,
    model: selectedModelName.value,
  }).catch((err) => message.warning(err));
  if (res.name) {
    knowledgeBases.value.push(res);
    knowledgeBaseStore.setCurKnowledgeBase(res);
  }
  newKnowledgeBaseName.value = "";
  selectedModelName.value = "";
};

const modelOptions = computed(() => {
  return appStore.getModels.map(model => {
    return {
      label: model.name,
      value: model.name,
    };
  });
});
</script>
<template>
  <a-space direction="vertical" class="list">
    <SideItem
      v-for="(knowledgeBase, index) in knowledgeBases"
      :key="index"
      :title="knowledgeBase.name"
      :callback="() => selectThisKnowledgeBase(knowledgeBase)"
      :selected="knowledgeBase.name === knowledgeBaseStore.curKnowledgeBase?.name"
    />
    <a-divider style="margin: 0" v-if="knowledgeBases.length !== 0" />
    <SideItem
      :title="$t('knowledge-base.new-knowledge-base')"
      :callback="() => openModal = !openModal"
    />
  </a-space>
  <a-modal
    v-model:open="openModal"
    :title="$t('knowledge-base.new-knowledge-base')"
    @ok="newKnowledgeBase"
  >
    <a-space direction="vertical" style="width: 100%">
      <div>
        <h3 class="para-title">
          {{ $t("knowledge-base.knowledge-base-name") }}
        </h3>
        <a-input v-model:value="newKnowledgeBaseName" placeholder="New Name" />
      </div>
      <div>
        <h3 class="para-title">{{ $t("knowledge-base.embedding-model") }}</h3>
        <a-select
          v-model:value="selectedModelName"
          style="width: 100%"
          :options="modelOptions"
        ></a-select>
      </div>
    </a-space>
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
