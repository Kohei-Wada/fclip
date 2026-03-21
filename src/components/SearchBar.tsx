export function SearchBar({
  query,
  onQueryChange,
  onKeyDown,
  resultCount,
  inputRef,
}: {
  query: string;
  onQueryChange: (q: string) => void;
  onKeyDown: (e: React.KeyboardEvent) => void;
  resultCount: number;
  inputRef: React.RefObject<HTMLInputElement | null>;
}) {
  return (
    <div className="search-bar">
      <span className="search-icon">&gt;</span>
      <input
        ref={inputRef}
        type="text"
        value={query}
        onChange={(e) => onQueryChange(e.target.value)}
        onKeyDown={onKeyDown}
        placeholder="Search clipboard history..."
        autoFocus
      />
      <span className="result-count">{resultCount}</span>
    </div>
  );
}
