<script setup lang="ts">
import SideItem from "@/components/SideItem.vue";
import { onMounted } from "vue";
onMounted(() => {
});

const newAssistant = async () => {
  console.log("你好");
};
</script>
<template>
  <a-space direction="vertical" class="list">
    <SideItem
      title="Test TitleTitleTitleTitleTitleTitleTitle"
      :showSettingIcon="true"
    />
    <a-divider style="margin: 0" />
    <SideItem
      :title="$t('chat.new-assistant')"
      :callback="newAssistant"
    />
  </a-space>
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
