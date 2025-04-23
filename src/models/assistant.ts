export interface AssistantInfo {
  id: number;
  uuid: string;
  name: string;
  model: string;
  contextNum: number;
}

export interface AssistantParameter {
  mirostat?: number;
  mirostat_eta?: number;
  mirostat_tau?: number;
  num_ctx?: number;
  repeat_last_n?: number;
  repeat_penalty?: number;
  temperature?: number;
  seed?: number;
  stop?: string;
  num_predict?: number;
  top_k?: number;
  top_p?: number;
  min_p?: number;
}
