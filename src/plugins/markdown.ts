import hljs from "highlight.js";
import MarkdownIt from "markdown-it";

const md = new MarkdownIt({
  html: true, // 允许 HTML 标签
  linkify: true, // 自动解析 URL 为链接
  typographer: true, // 启用排版
  highlight: (str: string, lang: string | undefined): string => {
    // 如果指定了语言并且 highlight.js 支持它
    if (lang && hljs.getLanguage(lang)) {
      try {
        // 使用 highlight.js 来高亮代码
        return `<pre class="hljs"><code>${hljs.highlight(str, { language: lang }).value}</code></pre>`;
      } catch (__) {
        // 如果出错，就返回原始代码
        return `<pre class="hljs"><code>${md.utils.escapeHtml(str)}</code></pre>`;
      }
    }
    // 如果没有语言或 highlight.js 不支持该语言，返回未高亮的代码
    return `<pre class="hljs"><code>${md.utils.escapeHtml(str)}</code></pre>`;
  },
});

export default md;
