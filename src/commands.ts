import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { SearchResult, Keybindings } from "./types";

// IPC command names
export const Commands = {
  SEARCH_CLIPBOARD: "search_clipboard",
  PASTE_ENTRY: "paste_entry",
  DELETE_ENTRY: "delete_entry",
  TOGGLE_PIN: "toggle_pin",
  GET_KEYBINDINGS: "get_keybindings",
  GET_THEME: "get_theme",
  OPEN_CONFIG: "open_config",
} as const;

// Window utilities
export function hideWindow() {
  return getCurrentWindow().hide();
}

export function showWindow() {
  const win = getCurrentWindow();
  return win.show().then(() => win.setFocus());
}

// Typed invoke helpers
export function searchClipboard(query: string, pinnedOnly: boolean | null) {
  return invoke<SearchResult[]>(Commands.SEARCH_CLIPBOARD, { query, pinnedOnly });
}

export function pasteEntry(id: number) {
  return invoke(Commands.PASTE_ENTRY, { id });
}

export function deleteEntry(id: number) {
  return invoke(Commands.DELETE_ENTRY, { id });
}

export function togglePin(id: number, label: string) {
  return invoke(Commands.TOGGLE_PIN, { id, label });
}

export function getKeybindings() {
  return invoke<Keybindings>(Commands.GET_KEYBINDINGS);
}

export function getTheme() {
  return invoke<string>(Commands.GET_THEME);
}

export function openConfig() {
  return invoke(Commands.OPEN_CONFIG);
}
