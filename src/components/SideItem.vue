<script setup lang="ts">
import { SettingOutlined } from "@ant-design/icons-vue";
import { h } from "vue";
const {
  title,
  showSettingIcon = false,
  selected = false,
  callback = () => {},
  buttonCallback = () => {},
} = defineProps<{
  title: string;
  showSettingIcon?: boolean;
  selected?: boolean;
  callback?: () => void;
  buttonCallback?: () => void;
}>();
</script>
<template>
  <div @click="callback">
    <a-flex
      justify="space-between"
      align="center"
      class="side-item"
      :class="{ 'selected': selected }"
    >
      <span
        style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis"
      >{{ title }}</span>
      <a-button
        v-if="showSettingIcon"
        type="text"
        shape="circle"
        :icon="h(SettingOutlined)"
        size="small"
        @click="buttonCallback"
      ></a-button>
    </a-flex>
  </div>
</template>
<style scoped>
.side-item {
  height: 37px;
  width: 100%;
  border-radius: 50px;
  padding: 0 10px;
  border: 1px solid transparent;
  transition: all 0.2s ease-in-out;
}

.selected, .side-item:hover {
  border: 1px solid var(--content-shadow);
  background-color: var(--content-select);
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}
</style>
