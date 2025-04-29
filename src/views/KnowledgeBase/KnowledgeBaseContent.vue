<script setup lang="ts">
import { KnowledgeBaseInfo } from "@/models/knowledge_base";
import { invoke } from "@tauri-apps/api/core";
import { once, UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { message } from "ant-design-vue";
import { ref, watch } from "vue";

const { knowledgeBase } = defineProps<{
  knowledgeBase: KnowledgeBaseInfo | null;
}>();
const fileActiveKey = ref("file");
const files = ref<string[]>([]);

const getCurKnowledgeBaseFiles = async () => {
  if (knowledgeBase !== null) {
    let res = await invoke<string[]>(
      "get_knowledge_base_files",
      {
        name: knowledgeBase.name,
      },
    ).catch((err) => message.warning(err));
    if (res.length !== 0) {
      files.value = res;
    }
  }
};

watch(() => knowledgeBase?.name, () => {
  files.value = [];
  getCurKnowledgeBaseFiles();
}, { immediate: true });

const openFileDialog = async () => {
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: "Files",
        extensions: ["pdf"],
      },
    ],
  });

  if (selected) {
    console.log("Selected file:", selected);
    // 在此处处理选中的文件路径，例如读取文件内容或传递给后端
  } else {
    console.log("No file selected");
  }
};

let dragDropEvent: UnlistenFn | null = null;

const mouseEnterDragger = async () => {
  dragDropEvent = await once("tauri://drag-drop", (event) => {
    console.log(event);
  });
};

const mouseLeaveDragger = () => {
  if (dragDropEvent) {
    dragDropEvent();
  }
};
</script>
<template>
  <div class="content-view">
    <a-collapse v-model:activeKey="fileActiveKey" class="">
      <a-collapse-panel key="file" :header="$t('knowledge-base.file')">
        <div
          @mouseenter="mouseEnterDragger"
          @mouseleave="mouseLeaveDragger"
        >
          <a-upload-dragger
            :openFileDialogOnClick="false"
            @click="openFileDialog"
          >
            <p class="ant-upload-text">{{ $t("upload-text") }}</p>
            <p class="ant-upload-hint">{{ $t("upload-hint") }}</p>
          </a-upload-dragger>
        </div>
      </a-collapse-panel>
    </a-collapse>
  </div>
</template>
<style scoped>
.content-view {
  height: 100%;
  padding: 10px 15px;
}
</style>
