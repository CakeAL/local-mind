<script setup lang="ts">
import MarkdownRender from "@/components/MarkdownRender.vue";
import { AssistantInfo } from "@/models/assistant";
import { CaretUpOutlined } from "@ant-design/icons-vue";
import { Channel, invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import { computed, h, onMounted, ref } from "vue";

type ChatResponseEvent =
  | {
    event: "Started";
    data: {
      userMessage: Message;
    };
  }
  | {
    event: "Progress";
    data: {
      model: string;
      createAt: Date;
      content: string;
    };
  }
  | {
    event: "Finished";
    data: {
      assistantMessage: Message;
    };
  };

const { assistant } = defineProps<{
  assistant: AssistantInfo | null;
}>();

const userInput = ref<string>("");
const sendButtonDisabled = computed(() => {
  return userInput.value.trim() === "";
});
const sendButtonLoading = ref<boolean>(false);
const conversation = ref<Message[]>([]);
const onEvent = new Channel<ChatResponseEvent>();

onEvent.onmessage = (message) => {
  console.log("Received event:", message);
  switch (message.event) {
    case "Started": {
      // 清空输入消息框的内容。
      userInput.value = "";
      const { userMessage } = message.data;
      conversation.value.push(userMessage);
      console.log("Chat started with user message:", userMessage);
      break;
    }
    case "Progress": {
      // [TODO]
      const { model, createAt, content } = message.data;
      console.log("Chat progress:", model, createAt, content);
      break;
    }
    case "Finished": {
      const { assistantMessage } = message.data;
      console.log("Chat finished with assistant message:", assistantMessage);
      conversation.value.push(assistantMessage);
      sendButtonLoading.value = false;
      break;
    }
    default: {
      console.warn("Unknown event:", message);
      break;
    }
  }
};

const userSendMessage = async () => {
  if (userInput.value.trim() === "" || assistant === null) {
    return;
  }
  // 设置按钮等待
  sendButtonLoading.value = true;
  await invoke("user_send_message", {
    assistantInfo: assistant,
    content: userInput.value,
    onEvent,
  }).catch((err) => {
    message.warning(err);
    sendButtonLoading.value = false;
  });
};
</script>
<template>
  <a-layout class="chat-content-view">
    <a-layout-content class="chat-history">
      <a-list item-layout="horizontal" :data-source="conversation" size="large">
        <template #renderItem="{ message }">
          <a-list-item>
            <a-list-item-meta
              description="Ant Design, a design language for background applications, is refined by Ant UED Team"
            >
              <template #title>
                {{ message.model }}
              </template>
              <template #avatar>
                <a-avatar src="https://joeschmoe.io/api/v1/random" />
              </template>
            </a-list-item-meta>
          </a-list-item>
        </template>
      </a-list>
    </a-layout-content>
    <a-layout-footer class="input-box">
      <a-card class="input-card" :bodyStyle="{ padding: '10px' }"><a-textarea
          v-model:value="userInput"
          style="padding: 0"
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
            :disabled="
              sendButtonDisabled || assistant === null
              || sendButtonLoading
            "
            :loading="sendButtonLoading"
            @click="userSendMessage"
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
