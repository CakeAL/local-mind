import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";
import { i18n } from "./plugins/i18n";

export function toFormatDateString(createdAt: Date) {
  let date = new Date(createdAt);
  const options: Intl.DateTimeFormatOptions = {
    month: "2-digit", // 月份（两位数）
    day: "2-digit", // 日（两位数）
    hour: "2-digit", // 小时（两位数）
    minute: "2-digit", // 分钟（两位数）
    hour12: false, // 24小时制（如果需要12小时制，可以设置为true）
  };
  return date.toLocaleString("en-US", options).replace(",", "");
}

export const copyToClipboard = async (content: string) => {
  await writeText(content);
  message.success(i18n.global.t("copied"));
};
