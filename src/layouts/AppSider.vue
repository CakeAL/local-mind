<template>
  <a-layout-sider
    class="sider"
    v-model:collapsed="uiStore.siderCollapsed"
    collapsedWidth="50"
  >
    <a-flex justify="space-between" vertical style="height: 100%">
      <a-menu
        v-model:openKeys="state.openKeys"
        v-model:selectedKeys="state.selectedKeys"
        mode="inline"
        :collapsed="uiStore.siderCollapsed"
        :items="items"
        class="menu"
      ></a-menu>
      <!-- 底部固定菜单项 -->
      <a-menu
        v-model:openKeys="state.openKeys"
        v-model:selectedKeys="state.selectedKeys"
        class="bottom-menu"
        mode="inline"
        :collapsed="uiStore.siderCollapsed"
        :items="bottomItems"
      ></a-menu></a-flex></a-layout-sider>
</template>

<style scoped>
.sider {
  text-align: center;
  line-height: 120px;
  background: transparent;
  user-select: none;
  height: 100%;
}

.sider .menu {
  background: transparent;
  overflow-y: auto;
  border: none;
}

.sider .bottom-menu {
  background: transparent;
  margin-top: auto;
  border: none;
}
</style>

<script setup lang="ts">
import {
  FolderOpenOutlined,
  MessageOutlined,
  SettingOutlined,
} from "@ant-design/icons-vue";
import { computed, h, reactive, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useUIStore } from "../stores/ui.ts";

const { t } = useI18n();
const uiStore = useUIStore();
const state = reactive({
  selectedKeys: ["1"],
  openKeys: ["sub1"],
  preOpenKeys: ["sub1"],
});

const items = reactive([
  {
    key: "chat",
    icon: () => h(MessageOutlined),
    label: computed(() => t("menu.chat")),
    title: computed(() => t("menu.chat")),
  },
  {
    key: "2",
    icon: () => h(FolderOpenOutlined),
    label: "Option 2",
    title: "Option 2",
  },
]);
// 底部固定菜单项
const bottomItems = reactive([
  {
    key: "setting",
    icon: () => h(SettingOutlined),
    label: computed(() => t("menu.setting")),
    title: computed(() => t("menu.setting")),
  },
]);
watch(
  () => state.openKeys,
  (_val, oldVal) => {
    state.preOpenKeys = oldVal;
  },
);
</script>
