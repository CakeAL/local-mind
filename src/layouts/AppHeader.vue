<script setup lang="ts">
import { useUIStore } from "@/stores/ui";
import { LeftCircleOutlined, RightCircleOutlined } from "@ant-design/icons-vue";
import { h } from "vue";

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
    <div class="decorum-tb" data-tauri-decorum-tb></div>
  </a-layout-header>
</template>

<style scoped>
.header {
  text-align: left;
  height: 40px;
  width: 100%;
  padding-inline: 80px 0;
  line-height: 40px;
  background: transparent;
  user-select: none;
}

.header .decorum-tb {
  float: right;
  height: 100%;
}
</style>
