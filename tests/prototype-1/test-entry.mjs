import { existsSync } from "node:fs"
import os from "node:os"
import path from "node:path"
import { spawn, spawnSync } from "node:child_process"

function cargoCommand() {
  return process.platform === "win32" ? "cargo.exe" : "cargo"
}

function createCargoTestStep(filter) {
  return {
    group: filter,
    command: [
      cargoCommand(),
      "test",
      "--test",
      "prototype_1_detection_test",
      filter,
      "--",
      "--exact",
    ],
    cwd: path.resolve("src-tauri"),
  }
}

function markerExists(root, markerPaths) {
  return markerPaths.some((markerPath) => existsSync(path.join(root, markerPath)))
}

function defaultCandidatesForPlatform() {
  if (process.platform === "win32") {
    const candidates = []
    if (process.env["PROGRAMFILES(X86)"]) {
      candidates.push(path.join(process.env["PROGRAMFILES(X86)"], "Steam"))
    }
    if (process.env.PROGRAMFILES) {
      candidates.push(path.join(process.env.PROGRAMFILES, "Steam"))
    }
    candidates.push("C:\\Program Files (x86)\\Steam")
    candidates.push("C:\\Program Files\\Steam")
    return candidates
  }

  const home = os.homedir()
  if (process.platform === "darwin") {
    return [path.join(home, "Library", "Application Support", "Steam")]
  }

  return []
}

function hasRealInstallByDefaultPath() {
  const markerPaths =
    process.platform === "darwin"
      ? ["registry.vdf", "Steam.AppBundle", path.join("config", "libraryfolders.vdf")]
      : [path.join("steamapps", "libraryfolders.vdf"), path.join("config", "libraryfolders.vdf")]

  return defaultCandidatesForPlatform().some(
    (candidate) => existsSync(candidate) && markerExists(candidate, markerPaths),
  )
}

function hasRealInstallByWindowsRegistry() {
  if (process.platform !== "win32") {
    return false
  }

  const result = spawnSync(
    "reg",
    ["query", "HKLM\\SOFTWARE\\WOW6432Node\\Valve\\Steam", "/v", "InstallPath"],
    { encoding: "utf8" },
  )

  if (result.status !== 0 || !result.stdout) {
    return false
  }

  const line = result.stdout
    .split(/\r?\n/)
    .find((value) => value.includes("InstallPath") && value.includes("REG_"))
  if (!line) {
    return false
  }

  const installPath = line.split(/REG_(?:SZ|EXPAND_SZ)/)[1]?.trim()
  return Boolean(installPath && existsSync(installPath))
}

export function hasRealInstall() {
  return hasRealInstallByDefaultPath() || hasRealInstallByWindowsRegistry()
}

export function createExecutionPlan() {
  if (process.platform !== "win32" && process.platform !== "darwin") {
    throw new Error("prototype-1 supports only Windows and macOS path detection")
  }

  const platformPrefix =
    process.platform === "win32"
      ? "prototype_1_windows"
      : process.platform === "darwin"
        ? "prototype_1_macos"
        : "prototype_1_macos"

  const plan = []

  if (hasRealInstall()) {
    plan.push(createCargoTestStep(`${platformPrefix}_real_install_detection_smoke`))
  } else {
    plan.push(createCargoTestStep(`${platformPrefix}_default_mock_detects_valid_root`))
    plan.push(createCargoTestStep(`${platformPrefix}_libraryfolders_mock_detects_library_root`))
  }

  if (process.platform === "win32") {
    plan.push(createCargoTestStep("prototype_1_windows_registry_mock_detects_valid_root"))
  }

  if (process.platform === "darwin") {
    plan.push(createCargoTestStep("prototype_1_macos_manual_invalid_missing_markers"))
  }

  return plan
}

export function runCommand(step) {
  return new Promise((resolve, reject) => {
    const child = spawn(step.command[0], step.command.slice(1), {
      cwd: step.cwd,
      stdio: "inherit",
      shell: false,
    })

    child.on("error", reject)
    child.on("close", (code) => resolve(code ?? 1))
  })
}

export async function runCli() {
  for (const step of createExecutionPlan()) {
    const code = await runCommand(step)
    if (code !== 0) {
      return code
    }
  }

  return 0
}
