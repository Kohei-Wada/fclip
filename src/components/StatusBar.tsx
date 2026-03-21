import type { Keybindings } from "../types";

export function StatusBar({ keybindings }: { keybindings: Keybindings | null }) {
  const text = keybindings
    ? `${keybindings.select}: select | ${keybindings.close}: close | ${keybindings.delete}: delete`
    : "";

  return (
    <div className="status-bar">
      <span>{text}</span>
    </div>
  );
}
