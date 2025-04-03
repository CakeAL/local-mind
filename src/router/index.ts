import Chat from "@/views/Chat.vue";
import File from "@/views/File.vue";
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
    path: "/file",
    name: "File",
    component: File,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
