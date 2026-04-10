import { useEffect, useRef, useState, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { SearchResult } from "../types";
import { useCursor } from "./useCursor";
import { searchClipboard, pasteEntry, deleteEntry, hideWindow, showWindow } from "../commands";

export function useClipboardSearch(pinnedOnly: boolean | null) {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);
  const queryRef = useRef(query);
  const pinnedOnlyRef = useRef(pinnedOnly);
  const cursor = useCursor(listRef);

  useEffect(() => {
    queryRef.current = query;
  }, [query]);

  useEffect(() => {
    pinnedOnlyRef.current = pinnedOnly;
  }, [pinnedOnly]);

  const search = useCallback(
    async (q: string, resetIndex = true) => {
      try {
        const res = await searchClipboard(q, pinnedOnlyRef.current);
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
    const init = async () => {
      await search("");
      await showWindow();
    };
    init();

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
  }, [query, pinnedOnly, search]);

  const handlePaste = async (id: number) => {
    try {
      await pasteEntry(id);
      await hideWindow();
    } catch (e) {
      console.error("Paste failed:", e);
    }
  };

  const handleDelete = async (id: number) => {
    try {
      await deleteEntry(id);
      const res = await searchClipboard(queryRef.current, pinnedOnlyRef.current);
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
