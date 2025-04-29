<script setup lang="ts">
import { useKnowledgeBaseStore } from "@/stores/knowledge_base";
import { onUnmounted } from "vue";
import KnowledgeBaseContent from "./KnowledgeBaseContent.vue";
import KnowledgeBaseSider from "./KnowledgeBaseSider.vue";
const knowledgeBaseStore = useKnowledgeBaseStore();
onUnmounted(() => {
  knowledgeBaseStore.clear();
});
</script>
<template>
  <a-layout class="knowledge-base-view">
    <a-layout-sider class="knowledge-base-sider">
      <KnowledgeBaseSider />
    </a-layout-sider>
    <a-layout-content class="knowledge-base-content">
      <KnowledgeBaseContent
        :knowledge-base="knowledgeBaseStore.getCurKnowledgeBase"
      />
    </a-layout-content>
  </a-layout>
</template>
<style scoped>
.knowledge-base-view {
  height: 100%;
  background: transparent;
}

.knowledge-base-sider {
  height: 100%;
  background: transparent;
  box-shadow: 1px 0 0.5px var(--content-shadow);
}

.knowledge-base-content {
  height: 100%;
}
</style>
