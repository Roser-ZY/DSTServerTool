import {
  Archive,
  ClipboardList,
  Package,
  Rocket,
} from "lucide-vue-next";

export const navigationItems = [
  {
    title: "初始化部署",
    to: "/setup",
    icon: Rocket,
  },
  {
    title: "模组更新",
    to: "/mods",
    icon: Package,
  },
  {
    title: "存档管理",
    to: "/saves",
    icon: Archive,
  },
  {
    title: "任务记录",
    to: "/tasks",
    icon: ClipboardList,
  },
] as const;
