import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export type SteamPathStatus = "valid" | "invalid" | "not_found";

export interface SteamPathDetectionResult {
  status: SteamPathStatus;
  path: string | null;
  source: string | null;
  validationErrors: string[];
}

export interface ModUpdateGate {
  canProceed: boolean;
  reason: string;
}

export interface SteamPathUiState {
  inputPath: string;
  errorMessage: string;
}

type InvokeFn = <T>(command: string, payload?: unknown) => Promise<T>;

export async function detectSteamPath(
  manualPath?: string,
  invokeFn: InvokeFn = (command, payload) =>
    invoke(command, (payload ?? undefined) as Record<string, unknown> | undefined),
): Promise<SteamPathDetectionResult> {
  const payload = manualPath ? { manualPath } : { manualPath: null };
  return invokeFn<SteamPathDetectionResult>("detect_paths", payload);
}

export function evaluateModUpdateGate(result: SteamPathDetectionResult | null): ModUpdateGate {
  if (!result) {
    return {
      canProceed: false,
      reason: "Steam 路径尚未检测，请先执行检测或手动输入。",
    };
  }

  if (result.status === "valid") {
    return { canProceed: true, reason: "" };
  }

  if (result.validationErrors.length > 0) {
    return { canProceed: false, reason: result.validationErrors.join("; ") };
  }

  return {
    canProceed: false,
    reason: "当前 Steam 路径无效，请修正后再继续。",
  };
}

export function selectPathForModUpdate(
  currentPath: string,
  result: SteamPathDetectionResult | null,
): string {
  if (result?.status === "valid" && result.path) {
    return result.path;
  }

  return currentPath;
}

export function deriveSteamPathUiState(
  currentPath: string,
  result: SteamPathDetectionResult | null,
): SteamPathUiState {
  if (result?.status === "valid" && result.path) {
    return {
      inputPath: result.path,
      errorMessage: "",
    };
  }

  if (result?.status === "not_found") {
    return {
      inputPath: "",
      errorMessage: "没有找到目录，请手动选择 Steam 目录。",
    };
  }

  return {
    inputPath: currentPath,
    errorMessage: "",
  };
}

export async function openSteamDirectory(defaultPath?: string): Promise<string | null> {
  void defaultPath;
  const selected = await open({
    directory: true,
    multiple: false,
  });

  if (!selected || Array.isArray(selected)) {
    return null;
  }

  return selected;
}

const STEAM_PATH_STORAGE_KEY = "dst.steamPath";

export function loadStoredSteamPath(): string {
  return window.localStorage.getItem(STEAM_PATH_STORAGE_KEY) ?? "";
}

export function saveStoredSteamPath(path: string): void {
  window.localStorage.setItem(STEAM_PATH_STORAGE_KEY, path);
}
