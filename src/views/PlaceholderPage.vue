<template>
  <section class="flex flex-1 items-center justify-center rounded-2xl border border-dashed bg-muted/30 px-6 py-16">
    <div class="space-y-2 text-center">
      <p class="text-sm font-medium uppercase tracking-[0.24em] text-muted-foreground">
        DSTServerTool
      </p>
      <h1 class="text-3xl font-semibold tracking-tight text-foreground">
        {{ title }}
      </h1>
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  title: string;
}>();
</script>
