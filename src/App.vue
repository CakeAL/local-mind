<script setup lang="ts">
import { onMounted } from "vue";
import AppHeader from "./components/AppHeader.vue";
import AppSider from "./components/AppSider.vue";
import { themeMedia, useUIStore } from "./store/ui";

const uiStore = useUIStore();
onMounted(() => {
  // 通过更新 themeReset 的值重新触发此 getter
  themeMedia.addEventListener("change", () => uiStore.toggleThemeReset());
});
</script>

<template>
  <a-config-provider :theme="uiStore.getThemeConfig" :locale="uiStore.getLocale">
    <a-layout class="app">
      <AppHeader></AppHeader>
      <a-layout style="background: transparent">
        <AppSider></AppSider><a-layout-content class="content">
          Content {{ $t('message.hello') }}
        </a-layout-content>
      </a-layout>
    </a-layout></a-config-provider>
</template>

<style scoped>
.content {
  text-align: center;
  line-height: 120px;
  color: #fff;
  background-color: #108ee9;
  border-radius: 8px 0 0 0;
}
</style>
