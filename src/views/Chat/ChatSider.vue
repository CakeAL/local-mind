<script setup lang="ts">
import SideItem from "@/components/SideItem.vue";
import { AssistantInfo, Parameter } from "@/models/assistant";
import { useAppStore } from "@/stores/app";
import { useChatStore } from "@/stores/chat";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import { computed, onMounted, ref } from "vue";

const assistants = ref<AssistantInfo[]>([]);
const openModal = ref(false);
const chatStore = useChatStore();
const appStore = useAppStore();
const selectedModelName = ref<String>("");

onMounted(() => {
  getAllAssistant().then(() => {
    if (assistants.value.length !== 0) {
      chatStore.setCurAssistant(assistants.value[0]);
    }
  });
});

const getAllAssistant = async () => {
  let res = await invoke<AssistantInfo[]>("get_all_assistant").catch(
    (err) => message.warning(err),
  );
  if (res.length !== 0) {
    assistants.value = res;
  }
};

const newAssistant = async () => {
  openModal.value = !openModal.value;
  if (!selectedModelName.value) {
    return;
  }
  let res = await invoke<AssistantInfo>("new_assistant", {
    model: selectedModelName.value,
  }).catch(
    (err) => message.warning(err),
  );
  if (res.uuid) {
    assistants.value.push(res);
    chatStore.setCurAssistant(res);
  }
  selectedModelName.value = "";
};

const selectThisAssistant = (assistant: AssistantInfo) => {
  chatStore.setCurAssistant(assistant);
};

const assistantConfigModal = ref<boolean>(false);
const assistantParameter = ref<Parameter | null>(null);
const assistantContextNum = ref<number>(5);
const openAssistantConfigModal = async (assistant: AssistantInfo) => {
  assistantParameter.value = null;
  assistantContextNum.value = 5;
  assistantParameter.value = await invoke<Parameter>(
    "get_assistant_config",
    {
      uuid: assistant.uuid,
    },
  ).catch((err) => message.warning(err));
  newAssistantName.value = assistant.name;
  assistantContextNum.value = assistant.context_num;
  assistantConfigModal.value = true;
};
const newAssistantName = ref<string>("");
const setNewAssistantConfig = async () => {
  await invoke("update_assistant_config", {
    uuid: chatStore.getCurAssistant?.uuid,
    name: newAssistantName.value,
    para: assistantParameter.value,
    contextNum: assistantContextNum.value,
  }).catch((err) => message.warning(err));
  getAllAssistant(); // 更新信息
  assistantConfigModal.value = false; // 关闭配置窗口
};

const deleteCurAssistant = async () => {
  await invoke("delete_assistant", {
    uuid: chatStore.getCurAssistant?.uuid,
  }).catch((err) => message.warning(err));
  assistants.value = assistants.value.filter(
    (assistant) => assistant.uuid !== chatStore.getCurAssistant?.uuid,
  ); // 过滤掉已经删除掉助手
  chatStore.setCurAssistant(assistants.value[0]);
  assistantConfigModal.value = false; // 关闭配置窗口
};

const assistantTemperature = computed({
  get: () => assistantParameter.value?.temperature ?? 1.0,
  set: (value) => {
    if (assistantParameter.value) {
      assistantParameter.value.temperature = value;
    }
  },
});
const assistantTopP = computed({
  get: () => assistantParameter.value?.top_p ?? 1.0,
  set: (value) => {
    if (assistantParameter.value) {
      assistantParameter.value.top_p = value;
    }
  },
});
const tempMarks = ref<Record<number, any>>({
  0.0: "0.0",
  0.7: "0.7",
  1.0: "1.0",
  2.0: "2.0",
});
const topPMarks = ref<Record<number, any>>({
  0.0: "0.0",
  1.0: "1.0",
});
const contextMarks = ref<Record<number, any>>({
  0: "0",
  5: "5",
  10: "10",
});
</script>
<template>
  <a-space direction="vertical" class="list">
    <SideItem
      v-for="(assistant, index) in assistants"
      :key="index"
      :title="assistant.name"
      :showSettingIcon="true"
      :callback="() => selectThisAssistant(assistant)"
      :selected="assistant.uuid === chatStore.curAssistant?.uuid"
      :buttonCallback="() => openAssistantConfigModal(chatStore.curAssistant!)"
    />
    <a-divider style="margin: 0" v-if="assistants.length !== 0" />
    <SideItem
      :title="$t('chat.new-assistant')"
      :callback="() => openModal = !openModal"
    />
  </a-space>
  <a-modal
    v-model:open="openModal"
    :title="$t('chat.select-model')"
    @ok="newAssistant"
  >
    <p v-if="appStore.modelListIsEmpty">
      {{ $t("chat.no-models") }}
    </p>
    <a-radio-group
      v-else
      v-model:value="selectedModelName"
      style="width: 100%"
      button-style="solid"
    >
      <a-radio-button
        v-for="(model, index) in appStore.getModels"
        :key="index"
        :value="model.name"
        class="radio-button"
      >{{ model.name }}</a-radio-button>
    </a-radio-group>
  </a-modal>
  <a-modal
    v-model:open="assistantConfigModal"
    :title="$t('chat.assistant-config')"
    @ok="setNewAssistantConfig"
    style="user-select: none"
  >
    <a-space direction="vertical" style="width: 100%">
      <a-flex justify="space-between" align="center">
        <h3 class="para-title">{{ $t("chat.model") }}</h3>
        <a-tag :bordered="false" color="processing" style="font-size: 1em">{{
          chatStore.curAssistant?.model
        }}</a-tag>
      </a-flex>
      <div>
        <h3 class="para-title">{{ $t("chat.assistant-name") }}</h3>
        <a-input v-model:value="newAssistantName" placeholder="New Name" />
      </div>
      <div>
        <h3 class="para-title">{{ $t("chat.temperature") }}</h3>
        <a-row style="width: 100%">
          <a-col :span="18">
            <a-slider
              v-model:value="assistantTemperature"
              :min="0.0"
              :max="2.0"
              :step="0.01"
              :marks="tempMarks"
            ><template #mark="{ label }">
                {{ label }}
              </template></a-slider>
          </a-col>
          <a-col :span="4">
            <a-input-number
              v-model:value="assistantTemperature"
              :min="0.0"
              :max="2.0"
              :step="0.01"
              style="margin-left: 16px"
            />
          </a-col>
        </a-row>
      </div>
      <div>
        <h3 class="para-title">Top-P</h3>
        <a-row style="width: 100%">
          <a-col :span="18">
            <a-slider
              v-model:value="assistantTopP"
              :min="0.0"
              :max="1.0"
              :step="0.01"
              :marks="topPMarks"
            ><template #mark="{ label }">
                {{ label }}
              </template></a-slider>
          </a-col>
          <a-col :span="4">
            <a-input-number
              v-model:value="assistantTopP"
              :min="0.0"
              :max="1.0"
              :step="0.01"
              style="margin-left: 16px"
            />
          </a-col>
        </a-row>
      </div>
      <div>
        <h3 class="para-title">{{ $t("chat.context-number") }}</h3>
        <a-row style="width: 100%">
          <a-col :span="18">
            <a-slider
              v-model:value="assistantContextNum"
              :min="0"
              :max="10"
              :step="1"
              :marks="contextMarks"
            ><template #mark="{ label }">
                {{ label }}
              </template></a-slider>
          </a-col>
          <a-col :span="4">
            <a-input-number
              v-model:value="assistantContextNum"
              :min="0"
              :max="10"
              :step="1"
              style="margin-left: 16px"
            />
          </a-col>
        </a-row>
      </div>
      <a-popconfirm
        :title="$t('chat.delete-assistant-message')"
        ok-text="Yes"
        cancel-text="No"
        @confirm="deleteCurAssistant"
      >
        <a-button type="primary" danger>{{
          $t("chat.delete-cur-assistant")
        }}</a-button></a-popconfirm>
    </a-space>
  </a-modal>
</template>
<style scoped>
.list {
  user-select: none;
  width: 100%;
  height: 100%;
  overflow-y: scroll;
  padding: 10px;
}

.radio-button {
  width: 100%;
  display: flex;
  border-radius: 5px;
  border: none;
  margin-bottom: 5px;
  box-sizing: border-box;
}

.radio-button::before {
  width: 0;
}

.para-title {
  margin-block-start: 5px;
  margin-block-end: 5px;
}
</style>
