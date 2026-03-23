import { useEffect, useRef, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { SearchResult } from "../types";

export function useClipboardSearch() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);
  const queryRef = useRef(query);

  useEffect(() => {
    queryRef.current = query;
  }, [query]);

  const search = useCallback(async (q: string) => {
    try {
      const res = await invoke<SearchResult[]>("search_clipboard", { query: q });
      setResults(res);
      setSelectedIndex(0);
    } catch (e) {
      console.error("Search failed:", e);
    }
  }, []);

  useEffect(() => {
    search("").then(() => {
      getCurrentWindow().show();
      getCurrentWindow().setFocus();
    });

    const unlisten = listen("clipboard-updated", () => {
      search(queryRef.current);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [search]);

  useEffect(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) {
        setQuery("");
        setResults([]);
        setSelectedIndex(0);
        search("");
        inputRef.current?.focus();
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [search]);

  useEffect(() => {
    const timer = setTimeout(() => search(query), 100);
    return () => clearTimeout(timer);
  }, [query, search]);

  useEffect(() => {
    const list = listRef.current;
    if (!list) return;
    const item = list.children[selectedIndex] as HTMLElement;
    if (item) {
      item.scrollIntoView({ block: "nearest" });
    }
  }, [selectedIndex]);

  const handlePaste = async (id: number) => {
    try {
      await invoke("paste_entry", { id });
      await getCurrentWindow().hide();
    } catch (e) {
      console.error("Paste failed:", e);
    }
  };

  const handleDelete = async (id: number) => {
    try {
      await invoke("delete_entry", { id });
      search(query);
    } catch (e) {
      console.error("Delete failed:", e);
    }
  };

  const refreshSearch = () => search(query);

  return {
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
  };
}
