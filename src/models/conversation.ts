import { AssistantInfo } from "./assistant";

export interface Message {
  id: number;
  assistant_uuid: string;
  model: string;
  created_at: Date;
  content: string;
  role: string;
  total_duration: number;
  load_duration: number;
  prompt_eval_count: number;
  prompt_eval_duration: number;
  eval_count: number;
  eval_duration: number;
}

export const newAssistantMessage = (assistant: AssistantInfo): Message => {
  return {
    id: 0,
    assistant_uuid: assistant.uuid,
    model: assistant.model,
    created_at: new Date(),
    content: "",
    role: "Assistant",
    total_duration: 0,
    load_duration: 0,
    prompt_eval_count: 0,
    prompt_eval_duration: 0,
    eval_count: 0,
    eval_duration: 0,
  } as Message;
};
