import { describe, it, expect } from "vitest";
import { clampIndex } from "./useCursor";

describe("clampIndex", () => {
  it("preserves index when within bounds", () => {
    expect(clampIndex(3, 10)).toBe(3);
  });

  it("clamps index to last item when beyond results length", () => {
    expect(clampIndex(5, 3)).toBe(2);
  });

  it("clamps to 0 when results are empty", () => {
    expect(clampIndex(5, 0)).toBe(0);
  });

  it("preserves index 0 with non-empty results", () => {
    expect(clampIndex(0, 5)).toBe(0);
  });

  it("preserves index at exact last position", () => {
    expect(clampIndex(4, 5)).toBe(4);
  });

  it("clamps index when results shrink by one", () => {
    expect(clampIndex(9, 9)).toBe(8);
  });
});
