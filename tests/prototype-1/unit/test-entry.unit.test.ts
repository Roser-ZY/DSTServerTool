import { describe, expect, it } from "vitest"

import { createExecutionPlan } from "../test-entry.mjs"

describe("createExecutionPlan", () => {
  it("returns executable cargo test steps", () => {
    const plan = createExecutionPlan()

    expect(plan.length).toBeGreaterThan(0)
    for (const step of plan) {
      expect(typeof step.group).toBe("string")
      expect(Array.isArray(step.command)).toBe(true)
      expect(step.command[0]).toMatch(/cargo(\.exe)?$/)
      expect(step.command).toContain("prototype_1_detection_test")
    }
  })
})
