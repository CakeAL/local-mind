<template>
  <a-layout-sider
    class="sider"
    v-model:collapsed="uiStore.siderCollapsed"
    collapsedWidth="50"
  >
    <a-flex justify="space-between" vertical style="height: 100%">
      <a-menu
        v-model:selectedKeys="state.selectedKeys"
        mode="inline"
        :collapsed="uiStore.siderCollapsed"
        :items="items"
        class="menu"
        @click="clickMenuItem"
      ></a-menu>
      <!-- 底部固定菜单项 -->
      <a-menu
        v-model:selectedKeys="state.selectedKeys"
        class="bottom-menu"
        mode="inline"
        :collapsed="uiStore.siderCollapsed"
        :items="bottomItems"
        @click="clickMenuItem"
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
import { useUIStore } from "@/stores/ui.ts";
import {
  FolderOpenOutlined,
  MessageOutlined,
  SettingOutlined,
} from "@ant-design/icons-vue";
import { computed, h, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

const { t } = useI18n();
const uiStore = useUIStore();
const router = useRouter();
const state = reactive({
  selectedKeys: ["chat"],
});

const items = reactive([
  {
    key: "chat",
    icon: () => h(MessageOutlined),
    label: computed(() => t("menu.chat")),
    title: computed(() => t("menu.chat")),
    path: "/chat",
  },
  {
    key: "knowledge-base",
    icon: () => h(FolderOpenOutlined),
    label: computed(() => t("menu.knowledge-base")),
    title: computed(() => t("menu.knowledge-base")),
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

const clickMenuItem = (
  { keyPath }: { item: Object; key: string; keyPath: string[] },
) => {
  router.push(`/${keyPath.join("/")}`);
};
</script>
