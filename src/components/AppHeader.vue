<script setup lang="ts">
import { LeftCircleOutlined, RightCircleOutlined } from "@ant-design/icons-vue";
import { h } from "vue";
import { useUIStore } from "../store/ui";

const uiStore = useUIStore();
const toggleSiderCollapsed = () => {
  uiStore.toggleSiderCollapsed();
};
</script>

<template>
  <a-layout-header class="header" data-tauri-drag-region>
    <a-button
      type="text"
      shape="circle"
      :icon="
        uiStore.siderCollapsed
        ? h(RightCircleOutlined)
        : h(LeftCircleOutlined)
      "
      @click="toggleSiderCollapsed"
    />
  </a-layout-header>
</template>

<style scoped>
.header {
  text-align: left;
  color: #fff;
  height: 40px;
  width: 100%;
  padding-inline: 80px;
  line-height: 40px;
  background-color: #7dbcea;
  user-select: none;
}
</style>
