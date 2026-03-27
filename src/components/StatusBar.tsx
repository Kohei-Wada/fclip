import type { Key, Keybindings } from "../types";

export function formatKey(k: Key): string {
  const parts = [];
  if (k.ctrl) parts.push("Ctrl");
  if (k.shift) parts.push("Shift");
  if (k.alt) parts.push("Alt");
  if (k.meta) parts.push("Meta");
  parts.push(
    k.key.length === 1 ? k.key.toUpperCase() : k.key.charAt(0).toUpperCase() + k.key.slice(1),
  );
  return parts.join("+");
}

export function StatusBar({ keybindings }: { keybindings: Keybindings | null }) {
  const fmt = (keys: Key[]) => (keys[0] ? formatKey(keys[0]) : "N/A");
  const text = keybindings
    ? `${fmt(keybindings.select)}: select | ${fmt(keybindings.close)}: close | ${fmt(keybindings.delete)}: delete | ${fmt(keybindings.help)}: help`
    : "";

  return (
    <div className="status-bar">
      <span>{text}</span>
    </div>
  );
}
