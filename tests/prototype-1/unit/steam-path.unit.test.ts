import { describe, expect, it } from "vitest";

import {
  detectSteamPath,
  deriveSteamPathUiState,
  evaluateModUpdateGate,
  selectPathForModUpdate,
  type SteamPathDetectionResult,
} from "../../../src/lib/steam-path";

describe("detectSteamPath", () => {
  it("passes manualPath to invoke payload", async () => {
    const result: SteamPathDetectionResult = {
      status: "valid",
      path: "D:/Steam",
      source: "manual",
      validationErrors: [],
    };

    const calls: Array<{ command: string; payload?: unknown }> = [];
    const invoke = async <T>(command: string, payload?: unknown): Promise<T> => {
      calls.push({ command, payload });
      return result as T;
    };

    const output = await detectSteamPath("D:/Steam", invoke);

    expect(output).toEqual(result);
    expect(calls).toEqual([
      {
        command: "detect_paths",
        payload: { manualPath: "D:/Steam" },
      },
    ]);
  });
});

describe("evaluateModUpdateGate", () => {
  it("blocks execution when detection is invalid", () => {
    const gate = evaluateModUpdateGate({
      status: "invalid",
      path: "D:/BrokenSteam",
      source: "manual",
      validationErrors: ["Missing required directory: steamapps/workshop"],
    });

    expect(gate.canProceed).toBe(false);
    expect(gate.reason).toContain("steamapps/workshop");
  });

  it("allows execution when detection is valid", () => {
    const gate = evaluateModUpdateGate({
      status: "valid",
      path: "D:/Steam",
      source: "manual",
      validationErrors: [],
    });

    expect(gate).toEqual({ canProceed: true, reason: "" });
  });
});

describe("selectPathForModUpdate", () => {
  it("prefills path from valid detection result", () => {
    const path = selectPathForModUpdate(
      "",
      {
        status: "valid",
        path: "D:/Steam",
        source: "windows-default",
        validationErrors: [],
      },
    );

    expect(path).toBe("D:/Steam");
  });

  it("keeps existing path when detection is not valid", () => {
    const path = selectPathForModUpdate(
      "E:/CustomSteam",
      {
        status: "invalid",
        path: "D:/Steam",
        source: "manual",
        validationErrors: ["broken"],
      },
    );

    expect(path).toBe("E:/CustomSteam");
  });
});

describe("deriveSteamPathUiState", () => {
  it("fills input when auto detection is valid", () => {
    const ui = deriveSteamPathUiState(
      "",
      {
        status: "valid",
        path: "D:/Steam",
        source: "windows-default",
        validationErrors: [],
      },
    );

    expect(ui).toEqual({ inputPath: "D:/Steam", errorMessage: "" });
  });

  it("keeps input empty and shows error when not found", () => {
    const ui = deriveSteamPathUiState(
      "",
      {
        status: "not_found",
        path: null,
        source: null,
        validationErrors: [],
      },
    );

    expect(ui.inputPath).toBe("");
    expect(ui.errorMessage).toContain("没有找到目录");
  });
});
