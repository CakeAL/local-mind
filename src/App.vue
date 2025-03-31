<script setup lang="ts">
import AppHeader from "./components/AppHeader.vue";
import AppSider from "./components/AppSider.vue";
import { useUIStore } from "./store/ui";
import variables from "./styles/variables.module.scss";

const uiStore = useUIStore();
</script>

<template>
  <a-config-provider :theme="uiStore.themeConfig">
    <a-layout class="app">
      <AppHeader></AppHeader>
      <a-layout style="background: transparent">
        <AppSider></AppSider><a-layout-content class="content">
          Content
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
