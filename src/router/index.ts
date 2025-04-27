import Chat from "@/views/Chat/Chat.vue";
import KnowledgeBase from "@/views/KnowledgeBase/KnowledgeBase.vue";
import Setting from "@/views/Setting.vue";
import { createRouter } from "vue-router";
import { createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    redirect: "/chat",
  },
  {
    path: "/chat",
    name: "Chat",
    component: Chat,
  },
  {
    path: "/setting",
    name: "Setting",
    component: Setting,
  },
  {
    path: "/knowledge-base",
    name: "KnowledgeBase",
    component: KnowledgeBase,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
