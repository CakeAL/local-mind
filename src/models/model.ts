// 对应模型信息
export interface ModelList {
  models: Model[];
}

export interface Model {
  name: string;
  model?: string; // 可选字段
  modified_at: string; // 使用 string 来表示日期字符串
  size: number; // 使用 number 来表示大小
  digest: string;
  details: ModelDetails;
}

export interface ModelDetails {
  format: string;
  family: string;
  families: string[];
  parameter_size: string;
  quantization_level: string;
}
