import type { Keybindings } from "../types";
import { formatKey } from "./StatusBar";

interface HelpEntry {
  keys: string;
  action: string;
}

function buildEntries(kb: Keybindings): HelpEntry[] {
  const fmt = (keys: typeof kb.select) => keys.map((k) => formatKey(k)).join(" / ");

  return [
    { keys: fmt(kb.select), action: "select" },
    { keys: fmt(kb.close), action: "close" },
    { keys: fmt(kb.delete), action: "delete" },
    { keys: fmt(kb.next), action: "next" },
    { keys: fmt(kb.prev), action: "prev" },
    { keys: fmt(kb.backspace), action: "backspace" },
    { keys: fmt(kb.clear), action: "clear query" },
    { keys: "Ctrl+F", action: "pin / unpin" },
    { keys: fmt(kb.toggle_theme), action: "toggle theme" },
    { keys: fmt(kb.help), action: "toggle help" },
  ];
}

export function HelpOverlay({ keybindings }: { keybindings: Keybindings }) {
  const entries = buildEntries(keybindings);

  return (
    <div className="help-overlay">
      <div className="help-content">
        <div className="help-title">Keybindings</div>
        {entries.map((e) => (
          <div className="help-row" key={e.action}>
            <span className="help-key">{e.keys}</span>
            <span className="help-action">{e.action}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
