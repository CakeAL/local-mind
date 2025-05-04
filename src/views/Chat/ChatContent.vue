<script setup lang="ts">
import MarkdownRender from "@/components/MarkdownRender.vue";
import { AssistantInfo } from "@/models/assistant";
import {
  Message,
  newAssistantMessage,
  SearchResult,
} from "@/models/conversation";
import { copyToClipboard, toFormatDateString } from "@/util";
import {
  CaretUpOutlined,
  CopyOutlined,
  DeleteOutlined,
  RetweetOutlined,
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
    event: "referenceContent";
    data: {
      searchResult: SearchResult[];
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

const newOnEvent = (messageIndex: number): Channel<ChatResponseEvent> => {
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
      case "referenceContent": {
        console.log(message.data.searchResult);

        if (messageIndex !== -1) {
          conversation.value[messageIndex].search_result =
            message.data.searchResult;
        } else {
          conversation.value[conversation.value.length - 1].search_result =
            message.data.searchResult;
        }
        break;
      }
      case "progress": {
        const { content } = message.data;
        if (messageIndex !== -1) {
          conversation.value[messageIndex].content += content;
        } else {
          conversation.value[conversation.value.length - 1].content += content;
          handleScrollBottom();
        }
        break;
      }
      case "finished": {
        const { assistantMessage } = message.data;
        if (messageIndex !== -1) {
          conversation.value[messageIndex] = assistantMessage;
        } else {
          conversation.value[conversation.value.length - 1] = assistantMessage;
          handleScrollBottom();
        }
        sendButtonLoading.value = false;
        break;
      }
      default: {
        console.warn("Unknown event:", message);
        break;
      }
    }
  };
  return onEvent;
};

const userSendMessage = async () => {
  let onEvent = newOnEvent(-1);
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

const removeMessageById = (messageId: number) => {
  conversation.value = conversation.value.filter(message =>
    message.id !== messageId
  );
};

const regenerateAssistantMessage = async (
  messageId: number,
  messageIndex: number,
) => {
  // // 如果点击的消息是用户消息，则需要重新生成在下一个地方
  // if (conversation.value[messageIndex].role == "User") {
  //   messageIndex += 1;
  //   if (messageIndex >= conversation.value.length) {
  //     // 如果没有下一个地方了需要新插入一个
  //     conversation.value.splice(newAssistantMessage(assistant!));
  //   }
  // }
  conversation.value[messageIndex].content = "";
  let onEvent = newOnEvent(messageIndex);
  // 设置按钮等待
  sendButtonLoading.value = true;
  await invoke("regenerate_assistant_message", {
    assistantInfo: assistant,
    messageId: messageId,
    onEvent,
  }).catch((err) => {
    message.warning(err);
    sendButtonLoading.value = false;
  });
};

const deleteMessage = async (messageId: number) => {
  await invoke("delete_message", {
    assistantInfo: assistant,
    messageId,
  }).catch((err) => {
    message.warning(err);
  });
  removeMessageById(messageId);
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
          <template #renderItem="{ item, index }">
            <a-list-item class="chat-list-item">
              <a-list-item-meta
                :description="toFormatDateString(item.created_at)"
                style="margin-block-end: 0"
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
              <MarkdownRender
                :content="item.content"
                :search-result="item.search_result"
              />
              <template
                #actions
                class="item-actions"
              >
                <a-flex
                  justify="space-between"
                  align="center"
                  style="width: 100%"
                >
                  <div>
                    <a-button
                      size="small"
                      class="action-button"
                      @click="copyToClipboard(item.content)"
                    >
                      <template #icon>
                        <CopyOutlined />
                      </template>
                    </a-button>
                    <a-popconfirm
                      :title="$t('chat.regenerate-message')"
                      ok-text="Yes"
                      cancel-text="No"
                      @confirm="
                        regenerateAssistantMessage(
                          item.id,
                          index,
                        )
                      "
                      v-if="item.role === 'Assistant'"
                    >
                      <a-button
                        size="small"
                        class="action-button"
                      >
                        <template #icon>
                          <RetweetOutlined />
                        </template>
                      </a-button>
                    </a-popconfirm>
                    <a-popconfirm
                      :title="$t('chat.delete-message')"
                      ok-text="Yes"
                      cancel-text="No"
                      @confirm="
                        deleteMessage(
                          item.id,
                        )
                      "
                    >
                      <a-button size="small" class="action-button">
                        <template #icon>
                          <DeleteOutlined />
                        </template>
                      </a-button>
                    </a-popconfirm>
                  </div>
                  <div v-if="item.role === 'Assistant'">
                    {{
                      (item.eval_count
                      / item.eval_duration * 1e9)
                      .toFixed(2)
                    }} Tokens/s
                  </div>
                </a-flex>
              </template>
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

.action-button {
  margin-right: 10px;
  color: rgba(127, 127, 127, 0.6);
}
</style>
