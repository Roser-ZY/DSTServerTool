import { createRouter, createWebHashHistory } from "vue-router";

import { navigationItems } from "@/lib/navigation";
import PlaceholderPage from "@/views/PlaceholderPage.vue";

const routes = [
  { path: "/", redirect: "/setup" },
  ...navigationItems.map((item) => ({
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
