import { describe, it, expect } from "vitest";
import { formatKey } from "./StatusBar";
import type { Key } from "../types";

function makeKey(
  key: string,
  mods: { ctrl?: boolean; shift?: boolean; alt?: boolean; meta?: boolean } = {},
): Key {
  return {
    key,
    ctrl: mods.ctrl ?? false,
    shift: mods.shift ?? false,
    alt: mods.alt ?? false,
    meta: mods.meta ?? false,
  };
}

describe("formatKey", () => {
  it("formats a simple key without modifiers", () => {
    expect(formatKey(makeKey("enter"))).toBe("Enter");
  });

  it("formats a single character key as uppercase", () => {
    expect(formatKey(makeKey("d"))).toBe("D");
  });

  it("formats Ctrl + single character", () => {
    expect(formatKey(makeKey("d", { ctrl: true }))).toBe("Ctrl+D");
  });

  it("formats multiple modifiers", () => {
    expect(formatKey(makeKey("v", { ctrl: true, shift: true }))).toBe("Ctrl+Shift+V");
  });

  it("formats Alt modifier", () => {
    expect(formatKey(makeKey("v", { alt: true }))).toBe("Alt+V");
  });

  it("formats Meta modifier", () => {
    expect(formatKey(makeKey("v", { meta: true }))).toBe("Meta+V");
  });

  it("formats all modifiers combined", () => {
    expect(formatKey(makeKey("a", { ctrl: true, shift: true, alt: true, meta: true }))).toBe(
      "Ctrl+Shift+Alt+Meta+A",
    );
  });

  it("capitalizes first letter of multi-char keys", () => {
    expect(formatKey(makeKey("escape"))).toBe("Escape");
    expect(formatKey(makeKey("backspace"))).toBe("Backspace");
  });
});
