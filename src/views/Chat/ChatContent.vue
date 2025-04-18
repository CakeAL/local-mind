<script setup lang="ts">
import MarkdownRender from "@/components/MarkdownRender.vue";
import { ref } from "vue";

const content = ref<string>(`
# Test Title
## Test Subtitle
- Test List Item
`);
</script>
<template>
  <MarkdownRender :content="content" />
</template>
<style scoped></style>
