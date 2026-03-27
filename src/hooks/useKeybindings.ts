import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Key, Keybindings } from "../types";

// Characters that require Shift to type (symbols on number/punctuation keys).
// When the configured key is one of these, we skip the strict shiftKey check
// because the user already implies Shift by specifying the symbol itself.
const SHIFTED_SYMBOLS = new Set('~!@#$%^&*()_+{}|:"<>?'.split(""));

export function matchesKeybinding(e: React.KeyboardEvent, bindings: Key[]): boolean {
  return bindings.some((b) => {
    if (e.ctrlKey !== b.ctrl || e.altKey !== b.alt || e.metaKey !== b.meta) return false;
    if (e.key.toLowerCase() !== b.key.toLowerCase()) return false;
    // For shifted symbols (e.g. "?"), don't require explicit Shift in config
    if (SHIFTED_SYMBOLS.has(b.key)) return true;
    return e.shiftKey === b.shift;
  });
}

export function useKeybindings() {
  const [keybindings, setKeybindings] = useState<Keybindings | null>(null);

  useEffect(() => {
    const load = async () => {
      const kb = await invoke<Keybindings>("get_keybindings");
      setKeybindings(kb);
    };
    load();
  }, []);

  return keybindings;
}
