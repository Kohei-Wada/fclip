import { useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { useClipboardSearch } from "./hooks/useClipboardSearch";
import { useKeybindings, matchesKeybinding } from "./hooks/useKeybindings";
import { SearchBar } from "./components/SearchBar";
import { ResultList } from "./components/ResultList";
import { StatusBar } from "./components/StatusBar";
import "./App.css";

function App() {
  const {
    query,
    setQuery,
    results,
    selectedIndex,
    setSelectedIndex,
    handlePaste,
    handleDelete,
    refreshSearch,
    inputRef,
    listRef,
  } = useClipboardSearch();

  const keybindings = useKeybindings();
  const [pinMode, setPinMode] = useState<{ id: number } | null>(null);
  const [pinLabel, setPinLabel] = useState("");

  const enterPinMode = (id: number) => {
    setPinMode({ id });
    setPinLabel("");
  };

  const confirmPin = () => {
    if (!pinMode) return;
    invoke("toggle_pin", { id: pinMode.id, label: pinLabel }).then(() => {
      setPinMode(null);
      setPinLabel("");
      refreshSearch();
    });
  };

  const cancelPin = () => {
    setPinMode(null);
    setPinLabel("");
    inputRef.current?.focus();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!keybindings) return;

    if (pinMode) {
      if (e.key === "Enter") {
        e.preventDefault();
        confirmPin();
      } else if (e.key === "Escape" || (e.ctrlKey && e.key === "[")) {
        e.preventDefault();
        cancelPin();
      }
      return;
    }

    if (e.ctrlKey && e.key === "[") {
      e.preventDefault();
      getCurrentWindow().hide();
      return;
    }

    if (e.ctrlKey && e.key === "h") {
      e.preventDefault();
      setQuery((q) => q.slice(0, -1));
      return;
    }

    if (e.ctrlKey && e.key === "u") {
      e.preventDefault();
      setQuery("");
      return;
    }

    if (matchesKeybinding(e, keybindings.next)) {
      e.preventDefault();
      setSelectedIndex((i) => (i + 1) % Math.max(results.length, 1));
    } else if (matchesKeybinding(e, keybindings.prev)) {
      e.preventDefault();
      setSelectedIndex((i) => (i - 1 + results.length) % Math.max(results.length, 1));
    } else if (matchesKeybinding(e, keybindings.select)) {
      e.preventDefault();
      if (results[selectedIndex]) {
        handlePaste(results[selectedIndex].id);
      }
    } else if (matchesKeybinding(e, keybindings.close)) {
      e.preventDefault();
      getCurrentWindow().hide();
    } else if (matchesKeybinding(e, keybindings.delete)) {
      if (results[selectedIndex] && !results[selectedIndex].pinned) {
        e.preventDefault();
        handleDelete(results[selectedIndex].id);
      }
    } else if (e.ctrlKey && e.key === "f") {
      if (results[selectedIndex]) {
        e.preventDefault();
        const current = results[selectedIndex];
        if (current.pinned) {
          invoke("toggle_pin", { id: current.id, label: "" }).then(() => refreshSearch());
        } else {
          enterPinMode(current.id);
        }
      }
    }
  };

  return (
    <div className="container">
      <div className="drag-handle" data-tauri-drag-region>
        <span className="app-title">fclip v{__APP_VERSION__}</span>
        <button className="close-btn" onClick={() => getCurrentWindow().hide()}>
          ×
        </button>
      </div>
      {pinMode ? (
        <div className="search-bar pin-mode">
          <span className="search-icon">📌</span>
          <input
            type="text"
            value={pinLabel}
            onChange={(e) => setPinLabel(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Label (optional, Enter to confirm)"
            autoFocus
          />
          <span className="result-count">Esc: cancel</span>
        </div>
      ) : (
        <SearchBar
          query={query}
          onQueryChange={setQuery}
          onKeyDown={handleKeyDown}
          resultCount={results.length}
          inputRef={inputRef}
        />
      )}
      <ResultList
        results={results}
        selectedIndex={selectedIndex}
        onPaste={handlePaste}
        onSelect={setSelectedIndex}
        listRef={listRef}
      />
      <StatusBar keybindings={keybindings} />
    </div>
  );
}

export default App;
