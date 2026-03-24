import type { Key, Keybindings } from "../types";

function formatKey(k: Key): string {
  const parts = [];
  if (k.ctrl) parts.push("Ctrl");
  if (k.shift) parts.push("Shift");
  if (k.alt) parts.push("Alt");
  if (k.meta) parts.push("Meta");
  parts.push(
    k.key.length === 1
      ? k.key.toUpperCase()
      : k.key.charAt(0).toUpperCase() + k.key.slice(1),
  );
  return parts.join("+");
}

export function StatusBar({ keybindings }: { keybindings: Keybindings | null }) {
  const text = keybindings
    ? `${formatKey(keybindings.select[0])}: select | ${formatKey(keybindings.close[0])}: close | ${formatKey(keybindings.delete[0])}: delete`
    : "";

  return (
    <div className="status-bar">
      <span>{text}</span>
    </div>
  );
}
