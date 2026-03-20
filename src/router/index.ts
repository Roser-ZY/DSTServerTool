import { createRouter, createWebHashHistory } from "vue-router";

import { navigationItems } from "@/lib/navigation";
import ModsView from "@/views/ModsView.vue";
import PlaceholderPage from "@/views/PlaceholderPage.vue";
import SetupView from "@/views/SetupView.vue";

const routes = [
  { path: "/", redirect: "/setup" },
  {
    path: "/setup",
    name: "setup",
    component: SetupView,
    meta: { title: "初始化部署" },
  },
  {
    path: "/mods",
    name: "mods",
    component: ModsView,
    meta: { title: "模组更新" },
  },
  ...navigationItems
    .filter((item) => item.to !== "/setup" && item.to !== "/mods")
    .map((item) => ({
      path: item.to,
      name: item.to.slice(1),
      component: PlaceholderPage,
      props: { title: item.title },
      meta: { title: item.title },
    })),
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
