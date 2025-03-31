<script setup lang="ts">
import { useUIStore } from "../store/ui.ts";

const uiStore = useUIStore();
</script>

<template>
  <a-layout-sider
    class="sider"
    v-model:collapsed="uiStore.siderCollapsed"
    collapsedWidth="50"
  >Sider</a-layout-sider>
</template>

<style scoped>
.sider {
  text-align: center;
  line-height: 120px;
  color: #fff;
  background-color: #3ba0e9;
  user-select: none;
}
</style>
