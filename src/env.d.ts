/// <reference types="@rsbuild/core/types" />
// parse-reasoning types
declare module "parse-reasoning" {
  export default function parseReasoning(content: string): Array<{
    type: string;
    content?: string;
  }>;
}
