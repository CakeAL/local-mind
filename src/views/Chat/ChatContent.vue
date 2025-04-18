<script setup lang="ts">
import MarkdownRender from "@/components/MarkdownRender.vue";
import { AssistantInfo } from "@/models/assistant";
import { CaretUpOutlined } from "@ant-design/icons-vue";
import { computed, h, ref } from "vue";

const { assistant } = defineProps<{
  assistant: AssistantInfo | null;
}>();

const content = ref<string>(`
# Test Title
## Test Subtitle
- Test List Item
`);
const userInput = ref<string>("");
const sendButtonDisabled = computed(() => {
  return userInput.value.trim() === "";
});
</script>
<template>
  <a-layout class="chat-content-view">
    <a-layout-content class="chat-history">
      <MarkdownRender :content="content" />
    </a-layout-content>
    <a-layout-footer class="input-box">
      <a-card class="input-card" :bodyStyle="{ padding: '10px' }"><a-textarea
          v-model:value="userInput"
          style="padding: 0;"
          :bordered="false"
          :placeholder="$t('chat.input-placeholder')"
          :auto-size="{ minRows: 1, maxRows: 10 }"
        />
        <a-flex class="buttons" justify="space-between" align="center">
          <div>
            <!-- [TODO] Other Buttons -->
          </div>
          <a-button
            type="primary"
            shape="circle"
            size="small"
            :icon="h(CaretUpOutlined)"
            style="font-size: 12px"
            :disabled="sendButtonDisabled"
          />
        </a-flex>
      </a-card>
    </a-layout-footer>
  </a-layout>
</template>
<style scoped>
.chat-content-view {
  background: transparent;
  height: 100%;
  padding: 10px 15px;
}

.input-box {
  background: transparent;
  padding: 0;
}

.input-card {
  width: 100%;
  padding: 0;
}

.buttons {
  width: 100%;
  margin-top: 5px;
}
</style>
