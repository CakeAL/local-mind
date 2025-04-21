<script setup lang="ts">
import md from "@/plugins/markdown";
import { ref, watch } from "vue";

const renderedHTML = ref<string>("");

const { content = "" } = defineProps<{
  content?: string;
}>();

watch(() => content, (newContent) => {
  renderedHTML.value = md.render(newContent);
}, { immediate: true });
</script>
<template>
  <div v-html="renderedHTML" class="markdown-body"></div>
</template>
<style scoped>
@import "highlight.js/styles/github-dark.css";
.markdown-body {
  padding-left: 48px;
  width: 99%;
  overflow-y: auto;
}
</style>
