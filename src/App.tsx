import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useClipboardSearch } from "./hooks/useClipboardSearch";
import { useKeybindings, matchesKeybinding } from "./hooks/useKeybindings";
import { SearchBar } from "./components/SearchBar";
import { ResultList } from "./components/ResultList";
import { StatusBar } from "./components/StatusBar";
import { HelpOverlay } from "./components/HelpOverlay";
import { TabBar, type Tab } from "./components/TabBar";
import { getTheme, togglePin, openConfig, hideWindow } from "./commands";
import "./App.css";

function App() {
  const keybindings = useKeybindings();
  const [pinMode, setPinMode] = useState<{ id: number } | null>(null);
  const [pinLabel, setPinLabel] = useState("");
  const [theme, setTheme] = useState<"dark" | "light">("dark");
  const [showHelp, setShowHelp] = useState(false);
  const [activeTab, setActiveTab] = useState<Tab>("recent");

  const pinnedOnly = activeTab === "pinned" ? true : false;

  const {
    query,
    setQuery,
    results,
    cursor,
    handlePaste,
    handleDelete,
    refreshSearch,
    inputRef,
    listRef,
  } = useClipboardSearch(pinnedOnly);

  useEffect(() => {
    const loadTheme = async () => {
      try {
        const mode = await getTheme();
        let resolved: "dark" | "light";
        if (mode === "system") {
          resolved = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
        } else {
          resolved = mode === "light" ? "light" : "dark";
        }
        setTheme(resolved);
        document.documentElement.dataset.theme = resolved;
      } catch (error) {
        console.error("Failed to get theme:", error);
        const fallback = window.matchMedia("(prefers-color-scheme: dark)").matches
          ? "dark"
          : "light";
        setTheme(fallback);
        document.documentElement.dataset.theme = fallback;
      }
    };
    loadTheme();
  }, []);

  useEffect(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) {
        setActiveTab("recent");
      }
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const enterPinMode = (id: number) => {
    setPinMode({ id });
    setPinLabel("");
  };

  const confirmPin = async () => {
    if (!pinMode) return;
    await togglePin(pinMode.id, pinLabel);
    setPinMode(null);
    setPinLabel("");
    refreshSearch();
  };

  const cancelPin = () => {
    setPinMode(null);
    setPinLabel("");
    inputRef.current?.focus();
  };

  const handleKeyDown = async (e: React.KeyboardEvent) => {
    if (!keybindings) return;

    if (pinMode) {
      if (e.key === "Enter") {
        e.preventDefault();
        confirmPin();
      } else if (e.key === "Escape" || (e.ctrlKey && e.key === "[")) {
        e.preventDefault();
        cancelPin();
      } else if (matchesKeybinding(e, keybindings.backspace)) {
        e.preventDefault();
        setPinLabel((l) => l.slice(0, -1));
      } else if (matchesKeybinding(e, keybindings.clear)) {
        e.preventDefault();
        setPinLabel("");
      } else if (e.ctrlKey || e.altKey || e.metaKey) {
        e.preventDefault();
      }
      return;
    }

    if (matchesKeybinding(e, keybindings.help)) {
      e.preventDefault();
      setShowHelp((s) => !s);
      return;
    }

    if (showHelp) {
      if (matchesKeybinding(e, keybindings.close) || (e.ctrlKey && e.key === "[")) {
        e.preventDefault();
        setShowHelp(false);
      }
      return;
    }

    if (matchesKeybinding(e, keybindings.tab_next)) {
      e.preventDefault();
      setActiveTab((t) => (t === "recent" ? "pinned" : "recent"));
      cursor.reset();
      return;
    }

    if (matchesKeybinding(e, keybindings.tab_prev)) {
      e.preventDefault();
      setActiveTab((t) => (t === "pinned" ? "recent" : "pinned"));
      cursor.reset();
      return;
    }

    if (e.ctrlKey && e.key === "[") {
      e.preventDefault();
      hideWindow();
      return;
    }

    if (matchesKeybinding(e, keybindings.backspace)) {
      e.preventDefault();
      setQuery((q) => q.slice(0, -1));
      return;
    }

    if (matchesKeybinding(e, keybindings.clear)) {
      e.preventDefault();
      setQuery("");
      return;
    }

    if (matchesKeybinding(e, keybindings.next)) {
      e.preventDefault();
      cursor.moveNext(results.length);
    } else if (matchesKeybinding(e, keybindings.prev)) {
      e.preventDefault();
      cursor.movePrev(results.length);
    } else if (matchesKeybinding(e, keybindings.select)) {
      e.preventDefault();
      if (results[cursor.selectedIndex]) {
        handlePaste(results[cursor.selectedIndex].id);
      }
    } else if (matchesKeybinding(e, keybindings.close)) {
      e.preventDefault();
      hideWindow();
    } else if (matchesKeybinding(e, keybindings.delete)) {
      if (results[cursor.selectedIndex] && !results[cursor.selectedIndex].pinned) {
        e.preventDefault();
        handleDelete(results[cursor.selectedIndex].id);
      }
    } else if (matchesKeybinding(e, keybindings.toggle_theme)) {
      e.preventDefault();
      const next = theme === "dark" ? "light" : "dark";
      setTheme(next);
      document.documentElement.dataset.theme = next;
    } else if (matchesKeybinding(e, keybindings.open_config)) {
      e.preventDefault();
      openConfig().catch((err) => console.error("Failed to open config:", err));
    } else if (e.ctrlKey && e.key === "f") {
      if (results[cursor.selectedIndex]) {
        e.preventDefault();
        const current = results[cursor.selectedIndex];
        if (current.pinned) {
          await togglePin(current.id, "");
          refreshSearch();
        } else {
          enterPinMode(current.id);
        }
      }
    } else if (e.ctrlKey || e.altKey || e.metaKey) {
      e.preventDefault();
    }
  };

  return (
    <div className="container">
      <div className="drag-handle" data-tauri-drag-region>
        <span className="app-title">fclip v{__APP_VERSION__}</span>
        <button className="close-btn" onClick={() => hideWindow()}>
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
      <TabBar activeTab={activeTab} />
      <div className="results-container">
        <ResultList
          results={results}
          selectedIndex={cursor.selectedIndex}
          onPaste={handlePaste}
          onSelect={cursor.selectByIndex}
          listRef={listRef}
        />
      </div>
      {showHelp && keybindings && <HelpOverlay keybindings={keybindings} />}
      <StatusBar keybindings={keybindings} />
    </div>
  );
}

export default App;
