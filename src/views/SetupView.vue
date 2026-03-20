<script setup lang="ts">
import { onMounted, ref } from "vue";
import SteamPathInput from "@/components/steam-path/SteamPathInput.vue";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  detectSteamPath,
  deriveSteamPathUiState,
  loadStoredSteamPath,
  openSteamDirectory,
  saveStoredSteamPath,
} from "@/lib/steam-path";

const pathInput = ref(loadStoredSteamPath());
const loading = ref(false);
const errorMessage = ref("");

async function detectAndApply(): Promise<void> {
  loading.value = true;
  try {
    const next = await detectSteamPath();
    const uiState = deriveSteamPathUiState(pathInput.value, next);
    pathInput.value = uiState.inputPath;
    errorMessage.value = uiState.errorMessage;

    if (next.status === "valid" && uiState.inputPath) {
      saveStoredSteamPath(uiState.inputPath);
    }
  } finally {
    loading.value = false;
  }
}

async function browseDirectory(): Promise<void> {
  try {
    const selected = await openSteamDirectory(pathInput.value);
    if (!selected) {
      return;
    }

    loading.value = true;
    const validation = await detectSteamPath(selected);
    pathInput.value = selected;

    if (validation.status === "valid") {
      const resolvedPath = validation.path ?? selected;
      pathInput.value = resolvedPath;
      errorMessage.value = "";
      saveStoredSteamPath(resolvedPath);
      return;
    }

    errorMessage.value =
      validation.validationErrors.length > 0
        ? validation.validationErrors.join("; ")
        : "当前路径不是有效的 Steam 安装目录，请重新选择。";
  } catch {
    errorMessage.value = "无法打开目录选择器，请确认在桌面应用模式运行并已授予权限。";
  } finally {
    loading.value = false;
  }
}

onMounted(async () => {
  await detectAndApply();
});
</script>

<template>
  <div class="mx-auto flex w-full max-w-3xl flex-1 flex-col gap-4">
    <Card>
      <CardHeader>
        <CardDescription class="text-xs uppercase tracking-[0.2em]">初始化部署</CardDescription>
        <CardTitle>Steam 路径检测</CardTitle>
      </CardHeader>
      <CardContent>
      <SteamPathInput
        v-model="pathInput"
        :loading="loading"
        :error-message="errorMessage"
        @browse="browseDirectory"
      />
      </CardContent>
    </Card>
  </div>
</template>
