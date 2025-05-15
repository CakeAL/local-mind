<script setup lang="ts">
import { useUIStore } from "@/stores/ui";
import { SelectProps } from "ant-design-vue";
import { onMounted, ref } from "vue";
const activeKey = ref("1");
const curLocale = ref("");
const darkMode = ref("");
const curThemeColor = ref("");
const uiStore = useUIStore();
const languageOption = ref<SelectProps["options"]>([
  {
    value: "zh_CN",
    label: "🇨🇳 汉语",
  },
  {
    value: "en_US",
    label: "🇺🇸 English",
  },
]);

const themeColorOption = ref<SelectProps["options"]>([
  {
    value: "red",
    label: "red",
  },
  {
    value: "orange",
    label: "orange",
  },
  {
    value: "yellow",
    label: "yellow",
  },
  {
    value: "green",
    label: "green",
  },
  {
    value: "cyan",
    label: "cyan",
  },
  {
    value: "blue",
    label: "blue",
  },
  {
    value: "purple",
    label: "purple",
  },
]);

onMounted(() => {
  curLocale.value = uiStore.locale;
  darkMode.value = uiStore.darkMode;
  curThemeColor.value = uiStore.themeName;
});

const languageChange = () => {
  uiStore.setLocale(curLocale.value);
};

const darkModeChange = () => {
  uiStore.toggleDarkMode(darkMode.value);
};

const themeColorChange = () => {
  uiStore.setThemeName(curThemeColor.value);
};
</script>
<template>
  <a-tabs v-model:activeKey="activeKey" class="setting-tabs">
    <a-tab-pane key="1" :tab="$t('setting.ui-setting')">
      <a-space direction="vertical" style="width: 100%">
        <a-flex justify="space-between" align="center">
          <h3>{{ $t("setting.language") }}</h3>
          <a-select
            ref="select"
            v-model:value="curLocale"
            style="width: 120px"
            :options="languageOption"
            @change="languageChange"
          ></a-select></a-flex>
        <a-flex justify="space-between" align="center">
          <h3>{{ $t("setting.dark-mode") }}</h3>
          <a-radio-group v-model:value="darkMode" @change="darkModeChange">
            <a-radio-button value="auto">{{
              $t("setting.auto")
            }}</a-radio-button>
            <a-radio-button value="light">{{
              $t("setting.light")
            }}</a-radio-button>
            <a-radio-button value="dark">{{
              $t("setting.dark")
            }}</a-radio-button>
          </a-radio-group>
        </a-flex><a-flex justify="space-between" align="center">
          <h3>{{ $t("setting.theme-color") }}</h3>
          <a-select
            ref="select"
            v-model:value="curThemeColor"
            style="width: 120px"
            :options="themeColorOption"
            @change="themeColorChange"
          ></a-select>
        </a-flex>
      </a-space>
    </a-tab-pane>
    <a-tab-pane key="2" tab="Ollama" force-render
    >Content of Tab Pane 2</a-tab-pane>
    <a-tab-pane key="3" tab="Tab 3">Content of Tab Pane 3</a-tab-pane>
  </a-tabs>
</template>
<style scoped>
.setting-tabs {
  padding: 0 10px;
  height: 100%;
}
</style>
