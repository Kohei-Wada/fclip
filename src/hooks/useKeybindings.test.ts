import { describe, it, expect } from "vitest";
import { matchesKeybinding } from "./useKeybindings";
import type { Key } from "../types";

function makeEvent(
  key: string,
  mods: { ctrl?: boolean; shift?: boolean; alt?: boolean; meta?: boolean } = {},
) {
  return {
    key,
    ctrlKey: mods.ctrl ?? false,
    shiftKey: mods.shift ?? false,
    altKey: mods.alt ?? false,
    metaKey: mods.meta ?? false,
  } as React.KeyboardEvent;
}

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

describe("matchesKeybinding", () => {
  it("matches a simple key without modifiers", () => {
    const e = makeEvent("Enter");
    expect(matchesKeybinding(e, [makeKey("enter")])).toBe(true);
  });

  it("matches a key with Ctrl modifier", () => {
    const e = makeEvent("d", { ctrl: true });
    expect(matchesKeybinding(e, [makeKey("d", { ctrl: true })])).toBe(true);
  });

  it("does not match when modifier is missing", () => {
    const e = makeEvent("d");
    expect(matchesKeybinding(e, [makeKey("d", { ctrl: true })])).toBe(false);
  });

  it("does not match when key is different", () => {
    const e = makeEvent("n", { ctrl: true });
    expect(matchesKeybinding(e, [makeKey("j", { ctrl: true })])).toBe(false);
  });

  it("matches the second binding in the array", () => {
    const e = makeEvent("j", { ctrl: true });
    const bindings = [makeKey("n", { ctrl: true }), makeKey("j", { ctrl: true })];
    expect(matchesKeybinding(e, bindings)).toBe(true);
  });

  it("does not match when no binding matches", () => {
    const e = makeEvent("x", { ctrl: true });
    const bindings = [makeKey("n", { ctrl: true }), makeKey("j", { ctrl: true })];
    expect(matchesKeybinding(e, bindings)).toBe(false);
  });

  it("returns false for empty bindings array", () => {
    const e = makeEvent("Enter");
    expect(matchesKeybinding(e, [])).toBe(false);
  });

  it("is case-insensitive on the event key", () => {
    const e = makeEvent("D", { ctrl: true });
    expect(matchesKeybinding(e, [makeKey("d", { ctrl: true })])).toBe(true);
  });

  it("does not match when extra modifier is pressed", () => {
    const e = makeEvent("d", { ctrl: true, shift: true });
    expect(matchesKeybinding(e, [makeKey("d", { ctrl: true })])).toBe(false);
  });
});
