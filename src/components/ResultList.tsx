import type { SearchResult } from "../types";
import { HighlightedText } from "./HighlightedText";

function formatDate(isoString: string): string {
  const d = new Date(isoString);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

export function ResultList({
  results,
  selectedIndex,
  onPaste,
  onSelect,
  listRef,
}: {
  results: SearchResult[];
  selectedIndex: number;
  onPaste: (id: number) => void;
  onSelect: (index: number) => void;
  listRef: React.RefObject<HTMLDivElement | null>;
}) {
  return (
    <div className="results" ref={listRef}>
      {results.map((result, index) => (
        <div
          key={result.id}
          className={`result-item ${index === selectedIndex ? "selected" : ""} ${result.pinned ? "pinned" : ""}`}
          onClick={() => onPaste(result.id)}
          onMouseEnter={() => onSelect(index)}
        >
          {result.pinned && <span className="pin-icon">📌</span>}
          {result.label && <span className="entry-label">[{result.label}]</span>}
          <HighlightedText result={result} />
          <span className="entry-time">{formatDate(result.created_at)}</span>
        </div>
      ))}
      {results.length === 0 && (
        <div className="empty">No entries found</div>
      )}
    </div>
  );
}
