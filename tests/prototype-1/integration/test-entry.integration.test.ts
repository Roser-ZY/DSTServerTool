import { describe, expect, it } from "vitest"

import { hasRealInstall } from "../test-entry.mjs"

describe("hasRealInstall", () => {
  it("returns a boolean", () => {
    expect(typeof hasRealInstall()).toBe("boolean")
  })
})
