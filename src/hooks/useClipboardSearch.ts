import { useEffect, useRef, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { SearchResult } from "../types";
import { useCursor } from "./useCursor";

export function useClipboardSearch() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);
  const queryRef = useRef(query);
  const cursor = useCursor(listRef);

  useEffect(() => {
    queryRef.current = query;
  }, [query]);

  const search = useCallback(
    async (q: string, resetIndex = true) => {
      try {
        const res = await invoke<SearchResult[]>("search_clipboard", { query: q });
        setResults(res);
        if (resetIndex) {
          cursor.reset();
        } else {
          cursor.clamp(res.length);
        }
      } catch (e) {
        console.error("Search failed:", e);
      }
    },
    [cursor.reset, cursor.clamp],
  );

  useEffect(() => {
    search("").then(() => {
      getCurrentWindow().show();
      getCurrentWindow().setFocus();
    });

    const unlisten = listen("clipboard-updated", () => {
      search(queryRef.current, false);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [search]);

  useEffect(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) {
        search(queryRef.current, false);
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
      const res = await invoke<SearchResult[]>("search_clipboard", {
        query: queryRef.current,
      });
      setResults(res);
      cursor.clamp(res.length);
    } catch (e) {
      console.error("Delete failed:", e);
    }
  };

  const refreshSearch = () => search(query);

  return {
    query,
    setQuery,
    results,
    cursor,
    handlePaste,
    handleDelete,
    refreshSearch,
    inputRef,
    listRef,
  };
}
