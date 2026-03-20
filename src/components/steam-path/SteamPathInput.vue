<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { CardDescription } from "@/components/ui/card";
import { Input } from "@/components/ui/input";

defineProps<{
  modelValue: string;
  loading: boolean;
  errorMessage: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
  (e: "browse"): void;
}>();
</script>

<template>
  <div class="space-y-3">
    <div class="space-y-1">
      <CardDescription class="text-sm font-medium text-foreground">
        Steam 根目录
      </CardDescription>
      <CardDescription class="text-xs">
        自动检测到目录会自动填入。也可点击按钮手动从本地文件系统选择。
      </CardDescription>
    </div>

    <div class="flex items-center gap-2">
      <Input
        :model-value="modelValue"
        :disabled="loading"
        placeholder="Steam 安装根目录"
        @update:model-value="(value) => emit('update:modelValue', String(value))"
      />
      <Button :disabled="loading" variant="outline" @click="emit('browse')">
        选择目录
      </Button>
    </div>

    <CardDescription v-if="errorMessage" class="text-xs text-red-600">
      {{ errorMessage }}
    </CardDescription>
  </div>
</template>
