import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Keybindings } from "../types";

function parseKeybinding(binding: string) {
  const parts = binding.split("+");
  const key = parts[parts.length - 1].toLowerCase();
  const has = (mod: string) => parts.some((p) => p.toLowerCase() === mod);
  return { key, ctrl: has("ctrl"), shift: has("shift"), alt: has("alt"), meta: has("meta") };
}

export function matchesKeybinding(e: React.KeyboardEvent, binding: string): boolean {
  const parsed = parseKeybinding(binding);
  return (
    e.ctrlKey === parsed.ctrl &&
    e.shiftKey === parsed.shift &&
    e.altKey === parsed.alt &&
    e.metaKey === parsed.meta &&
    e.key.toLowerCase() === parsed.key
  );
}

export function useKeybindings() {
  const [keybindings, setKeybindings] = useState<Keybindings | null>(null);

  useEffect(() => {
    invoke<Keybindings>("get_keybindings").then(setKeybindings);
  }, []);

  return keybindings;
}
