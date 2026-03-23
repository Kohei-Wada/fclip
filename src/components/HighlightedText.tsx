import type { SearchResult } from "../types";

export function HighlightedText({ result }: { result: SearchResult }) {
  return (
    <span className="entry-text">
      {result.display.segments.map((seg, i) =>
        seg.highlighted ? (
          <span key={i} className="match-highlight">
            {seg.text}
          </span>
        ) : (
          <span key={i}>{seg.text}</span>
        ),
      )}
    </span>
  );
}
