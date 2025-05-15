<script setup lang="ts">
import { KnowledgeBaseInfo } from "@/models/knowledge_base";
import { openFileLocation } from "@/util";
import { FolderOpenOutlined } from "@ant-design/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { once, UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { message } from "ant-design-vue";
import { computed, ref, watch } from "vue";

const { knowledgeBase } = defineProps<{
  knowledgeBase: KnowledgeBaseInfo | null;
}>();
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
    // console.log("Selected file:", selected);
    addFileToKnowledgeBase(selected);
  } else {
    console.log("No file selected");
  }
};

let dragDropEvent: UnlistenFn | null = null;
const mouseEnterDragger = async () => {
  dragDropEvent = await once("tauri://drag-drop", (event: any) => {
    addFileToKnowledgeBase(event.payload.paths);
  });
};

const spinning = ref<boolean>(false);
const addFileToKnowledgeBase = async (filePaths: string[]) => {
  spinning.value = true;
  let warnings = await invoke<string[]>("add_file_to_knowledge_base", {
    knowledgeBase: knowledgeBase!,
    filePaths,
  })
    .catch((err) => message.warning(err));
  console.log(warnings);
  getCurKnowledgeBaseFiles();
  spinning.value = false;
};

const mouseLeaveDragger = () => {
  if (dragDropEvent) {
    dragDropEvent();
  }
};

const fileName = computed(() => (filePath: string) => {
  return filePath.split(/[\\/]/).pop();
});
</script>
<template>
  <div class="content-view" v-if="knowledgeBase">
    <a-collapse :accordion="true" activeKey="file" class="">
      <a-collapse-panel key="file" :header="$t('knowledge-base.file')">
        <a-spin :spinning="spinning" v-if="spinning">
          <a-alert
            :message="$t('knowledge-base.embedding')"
            :description="$t('knowledge-base.embedding-message')"
          ></a-alert></a-spin>
        <div
          @mouseenter="mouseEnterDragger"
          @mouseleave="mouseLeaveDragger"
          v-else
        >
          <a-upload-dragger
            :openFileDialogOnClick="false"
            @click="openFileDialog"
          >
            <p class="ant-upload-text">{{ $t("upload-text") }}</p>
            <p class="ant-upload-hint">{{ $t("upload-hint") }}</p>
          </a-upload-dragger>
        </div>
        <a-list bordered :data-source="files" class="file-list">
          <template #renderItem="{ item }">
            <a-list-item>{{ fileName(item) }}
              <template #actions>
                <a-button
                  size="small"
                  class="action-button"
                  @click="openFileLocation(item)"
                >
                  <template #icon>
                    <FolderOpenOutlined />
                  </template>
                </a-button>
              </template>
            </a-list-item>
          </template>
        </a-list>
      </a-collapse-panel>
    </a-collapse>
  </div>
</template>
<style scoped>
.content-view {
  height: 100%;
  padding: 10px 15px;
}

.file-list {
  margin-top: 10px;
}

.action-button {
  margin-right: 10px;
  color: rgba(127, 127, 127, 0.6);
}
</style>
