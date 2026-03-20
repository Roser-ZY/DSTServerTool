<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import SteamPathInput from "@/components/steam-path/SteamPathInput.vue";
import {
  detectSteamPath,
  deriveSteamPathUiState,
  evaluateModUpdateGate,
  loadStoredSteamPath,
  openSteamDirectory,
  saveStoredSteamPath,
  type SteamPathDetectionResult,
} from "@/lib/steam-path";

const pathInput = ref(loadStoredSteamPath());
const detection = ref<SteamPathDetectionResult | null>(null);
const loading = ref(false);
const errorMessage = ref("");
const executionMessage = ref("");

function applyValidationResult(result: SteamPathDetectionResult, fallbackPath: string): void {
  detection.value = result;

  if (result.status === "valid") {
    const resolvedPath = result.path ?? fallbackPath;
    pathInput.value = resolvedPath;
    errorMessage.value = "";
    saveStoredSteamPath(resolvedPath);
    return;
  }

  pathInput.value = fallbackPath;
  errorMessage.value =
    result.validationErrors.length > 0
      ? result.validationErrors.join("; ")
      : "当前路径不是有效的 Steam 安装目录，请重新选择。";
}

async function detectAndApply(): Promise<void> {
  loading.value = true;
  executionMessage.value = "";
  try {
    const next = await detectSteamPath();
    detection.value = next;
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
    executionMessage.value = "";
    const validation = await detectSteamPath(selected);
    applyValidationResult(validation, selected);
  } catch {
    errorMessage.value = "无法打开目录选择器，请确认在桌面应用模式运行并已授予权限。";
  } finally {
    loading.value = false;
  }
}

async function runModUpdate(): Promise<void> {
  executionMessage.value = "";

  const candidatePath = pathInput.value.trim();
  if (candidatePath) {
    loading.value = true;
    try {
      const validation = await detectSteamPath(candidatePath);
      applyValidationResult(validation, candidatePath);
    } finally {
      loading.value = false;
    }
  }

  const gate = evaluateModUpdateGate(detection.value);
  if (!gate.canProceed) {
    executionMessage.value = `无法开始模组更新：${gate.reason}`;
    return;
  }

  executionMessage.value = `已通过校验，使用路径 ${pathInput.value} 启动模组更新（占位流程）。`;
}

onMounted(async () => {
  await detectAndApply();
});
</script>

<template>
  <div class="mx-auto flex w-full max-w-3xl flex-1 flex-col gap-4">
    <Card>
      <CardHeader>
        <CardDescription class="text-xs uppercase tracking-[0.2em]">模组更新</CardDescription>
        <CardTitle>Steam 路径与更新入口</CardTitle>
      </CardHeader>
      <CardContent>
      <SteamPathInput
        v-model="pathInput"
        :loading="loading"
        :error-message="errorMessage"
        @browse="browseDirectory"
      />

      <div class="mt-4 flex flex-col gap-2">
        <Button
          :disabled="loading"
          @click="runModUpdate"
        >
          开始模组更新
        </Button>
        <CardDescription
          v-if="executionMessage"
          class="text-xs"
        >
          {{ executionMessage }}
        </CardDescription>
      </div>
      </CardContent>
    </Card>
  </div>
</template>
