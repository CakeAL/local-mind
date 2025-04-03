<script setup lang="ts">
import { onMounted } from "vue";
import AppContent from "./layouts/AppContent.vue";
import AppHeader from "./layouts/AppHeader.vue";
import AppSider from "./layouts/AppSider.vue";
import { themeMedia, useUIStore } from "./stores/ui";

const uiStore = useUIStore();
onMounted(() => {
  // 通过更新 themeReset 的值重新触发此 getter
  themeMedia.addEventListener("change", () => uiStore.toggleThemeReset());
});
</script>

<template>
  <a-config-provider
    :theme="uiStore.getThemeConfig"
    :locale="uiStore.getLocale"
  >
    <a-layout class="app">
      <AppHeader />
      <a-layout style="background: transparent">
        <AppSider />
        <AppContent />
      </a-layout>
    </a-layout></a-config-provider>
</template>

<style scoped>
</style>
