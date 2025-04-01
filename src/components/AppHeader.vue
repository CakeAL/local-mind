<script setup lang="ts">
import { LeftCircleOutlined, RightCircleOutlined } from "@ant-design/icons-vue";
import { h, nextTick, onMounted } from "vue";
import { useUIStore } from "../store/ui";

const uiStore = useUIStore();
const toggleSiderCollapsed = () => {
  uiStore.toggleSiderCollapsed();
};

onMounted(() => {
  // 用来更新 decorum
  const tbEl = document.querySelector("[data-tauri-decorum-tb]");
  if (!tbEl) return;
  // 制造一个 DOM 变化来触发观察器
  const temp = document.createElement("div");
  tbEl.appendChild(temp);
  setTimeout(() => temp.remove(), 10);
});
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
