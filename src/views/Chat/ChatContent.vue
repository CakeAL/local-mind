<script setup lang="ts">
import MarkdownRender from "@/components/MarkdownRender.vue";
import { AssistantInfo } from "@/models/assistant";
import { Message, newAssistantMessage } from "@/models/conversation";
import { toFormatDateString } from "@/util";
import {
  CaretUpOutlined,
  RobotOutlined,
  UserOutlined,
} from "@ant-design/icons-vue";
import { Channel, invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import { computed, h, nextTick, ref, watch } from "vue";

type ChatResponseEvent =
  | {
    event: "started";
    data: {
      userMessage: Message;
    };
  }
  | {
    event: "progress";
    data: {
      model: string;
      createAt: Date;
      content: string;
    };
  }
  | {
    event: "finished";
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

const getCurAssistantConversation = async () => {
  if (assistant !== null) {
    let res = await invoke<Message[]>("get_assistant_conversation", {
      assistantUuid: assistant.uuid,
    }).catch((err) => message.warning(err));
    if (res.length !== 0) {
      conversation.value = res.reverse();
    }
  }
};

watch(() => assistant?.uuid, () => {
  conversation.value = [];
  getCurAssistantConversation().then(() => handleScrollBottom());
}, { immediate: true });

const userSendMessage = async () => {
  let onEvent = new Channel<ChatResponseEvent>();

  onEvent.onmessage = (message) => {
    switch (message.event) {
      case "started": {
        // 清空输入消息框的内容。
        userInput.value = "";
        const { userMessage } = message.data;
        conversation.value.push(userMessage);
        let assistantMessage = newAssistantMessage(assistant!);
        conversation.value.push(assistantMessage);
        handleScrollBottom();
        break;
      }
      case "progress": {
        // [TODO]
        const { content } = message.data;
        conversation.value[conversation.value.length - 1].content += content;
        handleScrollBottom();
        break;
      }
      case "finished": {
        const { assistantMessage } = message.data;
        conversation.value[conversation.value.length - 1] = assistantMessage;
        sendButtonLoading.value = false;
        handleScrollBottom();
        break;
      }
      default: {
        console.warn("Unknown event:", message);
        break;
      }
    }
  };

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

// 滚动
// 用来引用滚动容器
const scrollDiv = ref<HTMLElement | null>(null);

// 滚动到顶部
// const handleScrollTop = () => {
//   nextTick(() => {
//     const scrollElem = scrollDiv.value;
//     if (scrollElem) {
//       scrollElem.scrollTo({ top: 0, behavior: "smooth" });
//     }
//   });
// };

// 滚动到底部
const handleScrollBottom = () => {
  if (!scrollDiv.value) return;
  nextTick(() => {
    const scrollElem = scrollDiv.value;
    if (scrollElem) {
      scrollElem.scrollTo({
        top: scrollElem.scrollHeight,
        behavior: "instant",
      });
    }
  });
};
</script>
<template>
  <a-layout class="chat-content-view">
    <a-layout-content class="chat-history">
      <div class="chat-list" ref="scrollDiv">
        <a-list
          item-layout="vertical"
          :data-source="conversation"
        >
          <template #renderItem="{ item }">
            <a-list-item class="chat-list-item">
              <a-list-item-meta
                :description="toFormatDateString(item.created_at)"
              >
                <template #title v-if="item.role === 'User'">
                  {{ $t("chat.user") }}
                </template>
                <template #title v-else-if="item.role === 'Assistant'">{{
                  item.model
                }}</template>
                <template #avatar>
                  <a-avatar
                    v-if="item.role === 'User'"
                    style="background-color: #87d068"
                  >
                    <template #icon><UserOutlined /></template>
                  </a-avatar>
                  <a-avatar
                    v-else-if="item.role === 'Assistant'"
                    style="background-color: #4175ef"
                  >
                    <template #icon><RobotOutlined /></template>
                  </a-avatar>
                </template>
              </a-list-item-meta>
              <MarkdownRender :content="item.content" />
            </a-list-item>
          </template>
        </a-list>
      </div>
    </a-layout-content>
    <a-layout-footer class="input-box">
      <a-card class="input-card" :bodyStyle="{ padding: '10px' }"><a-textarea
          v-model:value="userInput"
          style="padding: 0"
          :bordered="false"
          :placeholder="$t('chat.input-placeholder')"
          :auto-size="{ minRows: 1, maxRows: 10 }"
          @pressEnter="userSendMessage"
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

.chat-list {
  height: 100%;
  /* overflow: hidden; */
  padding-left: 0;
  overflow-y: scroll;
}

.chat-list-item {
  padding: 6px 12px 6px 0;
}
</style>
