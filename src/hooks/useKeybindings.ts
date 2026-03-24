import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Key, Keybindings } from "../types";

export function matchesKeybinding(e: React.KeyboardEvent, bindings: Key[]): boolean {
  return bindings.some(
    (b) =>
      e.ctrlKey === b.ctrl &&
      e.shiftKey === b.shift &&
      e.altKey === b.alt &&
      e.metaKey === b.meta &&
      e.key.toLowerCase() === b.key,
  );
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
