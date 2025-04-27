<script setup lang="ts">
import { useChatStore } from "@/stores/chat";
import { onUnmounted } from "vue";
import ChatContent from "./ChatContent.vue";
import ChatSider from "./ChatSider.vue";
const chatStore = useChatStore();
onUnmounted(() => {
  chatStore.clear();
});
</script>
<template>
  <a-layout class="chat-view">
    <a-layout-sider class="chat-sider">
      <ChatSider />
    </a-layout-sider>
    <a-layout-content class="chat-content">
      <ChatContent :assistant="chatStore.getCurAssistant" />
    </a-layout-content>
  </a-layout>
</template>
<style scoped>
.chat-view {
  height: 100%;
  background: transparent;
}

.chat-sider {
  height: 100%;
  background: transparent;
  box-shadow: 1px 0 0.5px var(--content-shadow);
}

.chat-content {
  height: 100%;
}
</style>
