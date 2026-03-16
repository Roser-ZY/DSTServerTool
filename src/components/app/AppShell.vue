<script setup lang="ts">
import { computed } from "vue";
import { RouterView, useRoute } from "vue-router";
import AppSidebar from "@/components/app/AppSidebar.vue";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
} from "@/components/ui/breadcrumb";
import { Separator } from "@/components/ui/separator";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";

const route = useRoute();

const currentTitle = computed(() => {
  const title = route.meta.title;
  return typeof title === "string" ? title : "DSTServerTool";
});
</script>

<template>
  <SidebarProvider>
    <AppSidebar />

    <SidebarInset>
      <header class="flex h-16 shrink-0 items-center gap-2 border-b bg-background/95 px-4 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <SidebarTrigger class="-ml-1" />
        <Separator orientation="vertical" class="mr-2 h-4" />
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbPage>{{ currentTitle }}</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
      </header>

      <div class="flex flex-1 flex-col p-6 lg:p-8">
        <RouterView />
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
