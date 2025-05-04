<script setup lang="ts">
import { SearchResult } from "@/models/conversation";
import md from "@/plugins/markdown";
import { copyToClipboard } from "@/util";
import { openFileLocation } from "@/util";
import { CopyOutlined, FolderOpenOutlined } from "@ant-design/icons-vue";
import parseReasoning from "parse-reasoning";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

const renderedHTML = ref<string>("");

const { content = "", searchResult } = defineProps<{
  content?: string;
  searchResult?: SearchResult[];
}>();

const { t } = useI18n();
const thought = ref<string>("");
const text = ref<string>("");
const activeKey = ref<number>(0);

watch(() => content, (newContent) => {
  if (newContent.length === 0) {
    // 说明重新生成了，需要清空一下已有内容。
    thought.value = "";
    text.value = "";
  }
  parseReasoning(newContent).map((e, _) => {
    if (e.type === "reasoning" && e.content) {
      // reasoning过程中的内容，可以用Collapse包裹。
      thought.value = e.content;
    } else if (e.content) {
      // ai模型返回的主内容
      text.value = e.content;
    }
  });
}, { immediate: true });

watch(text, text => {
  renderedHTML.value = md.render(text);
}, { immediate: true });

const thinking = computed(() => {
  if (thought.value.length !== 0 && text.value.length === 0) {
    activeKey.value = 1; // 思考时默认打开折叠面板
    return t("thinking");
  } else {
    activeKey.value = 0;
    return t("thought");
  }
});
const fileName = computed(() => (filePath: string) => {
  return filePath.split(/[\\/]/).pop();
});
</script>
<template>
  <div class="markdown-body">
    <a-collapse
      v-model:activeKey="activeKey"
      style="margin: 5px 0"
      v-if="thought.length !== 0 || searchResult"
    >
      <a-collapse-panel
        key="file"
        :header="$t('chat.reference')"
        v-if="searchResult"
      >
        <a-list
          bordered
          :data-source="searchResult"
          class="file-list"
          size="small"
        >
          <template #renderItem="{ item }">
            <a-list-item>
              <a-popover>
                <template #content>{{ item.content }}</template>
                {{
                  fileName(
                    item.file_path,
                  )
                }}</a-popover>
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
      <a-collapse-panel key="1" :header="thinking" v-if="thought.length !== 0">
        {{ thought }}
        <template #extra>
          <a-spin v-if="thinking === t('thinking')" />
          <a-button
            v-else
            size="small"
            class="action-button"
            @click="copyToClipboard(thought)"
          >
            <template #icon>
              <CopyOutlined />
            </template>
          </a-button></template>
      </a-collapse-panel>
    </a-collapse>
    <div v-html="renderedHTML"></div>
  </div>
</template>
<style scoped>
@import "highlight.js/styles/github-dark.css";
.markdown-body {
  padding-left: 48px;
  width: 99%;
  overflow-y: auto;
}
</style>
