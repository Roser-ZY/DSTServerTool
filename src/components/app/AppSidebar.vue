<script setup lang="ts">
import { computed } from "vue";
import { RouterLink, useRoute } from "vue-router";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
  SidebarSeparator,
} from "@/components/ui/sidebar";
import { navigationItems } from "@/lib/navigation";
import { Server } from "lucide-vue-next";

const route = useRoute();

const activePath = computed(() => route.path);
</script>

<template>
  <Sidebar collapsible="icon" variant="inset">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton as-child size="lg" tooltip="DSTServerTool">
            <RouterLink to="/setup">
              <div class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                <Server class="size-4" />
              </div>
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">DSTServerTool</span>
              </div>
            </RouterLink>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>

    <SidebarSeparator />

    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>导航</SidebarGroupLabel>
        <SidebarMenu>
          <SidebarMenuItem
            v-for="item in navigationItems"
            :key="item.to"
          >
            <SidebarMenuButton
              as-child
              :is-active="activePath === item.to"
              :tooltip="item.title"
            >
              <RouterLink :to="item.to">
                <component :is="item.icon" />
                <span>{{ item.title }}</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton class="pointer-events-none" size="lg">
            <Avatar class="h-8 w-8 rounded-lg">
              <AvatarFallback class="rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                DS
              </AvatarFallback>
            </Avatar>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-medium">DSTServerTool</span>
              <span class="truncate text-xs text-sidebar-foreground/70">基础界面框架</span>
            </div>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>

    <SidebarRail />
  </Sidebar>
</template>
