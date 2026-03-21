import type { SearchResult } from "../types";

export function HighlightedText({ result }: { result: SearchResult }) {
  const text = result.content;
  const maxLen = 100;
  const display = text.length > maxLen ? text.slice(0, maxLen) + "..." : text;

  if (result.match_indices.length === 0) {
    return <span className="entry-text">{display}</span>;
  }

  const indices = new Set(result.match_indices.filter((i) => i < display.length));
  const segments: { text: string; highlighted: boolean }[] = [];
  let current = { text: "", highlighted: indices.has(0) };

  for (let i = 0; i < display.length; i++) {
    const isHighlighted = indices.has(i);
    if (isHighlighted !== current.highlighted) {
      if (current.text) segments.push(current);
      current = { text: display[i], highlighted: isHighlighted };
    } else {
      current.text += display[i];
    }
  }
  if (current.text) segments.push(current);

  return (
    <span className="entry-text">
      {segments.map((seg, i) =>
        seg.highlighted ? (
          <span key={i} className="match-highlight">{seg.text}</span>
        ) : (
          <span key={i}>{seg.text}</span>
        )
      )}
    </span>
  );
}
