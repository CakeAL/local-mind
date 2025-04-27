export interface KnowledgeBase {
  id: number;
  name: string;
  model: string;
  requestTextNum: number;
  segmentingSize: number;
  matchThreshold: number;
  filePaths: string[];
}

export interface KnowledgeBaseInfo {
  id: number;
  name: string;
  model: string;
}
