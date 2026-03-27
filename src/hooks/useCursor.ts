import { useCallback, useEffect, useState } from "react";

export function clampIndex(prev: number, length: number): number {
  return Math.min(prev, Math.max(length - 1, 0));
}

export function useCursor(listRef: React.RefObject<HTMLDivElement | null>) {
  const [selectedIndex, setSelectedIndex] = useState(0);

  useEffect(() => {
    const list = listRef.current;
    if (!list) return;
    const item = list.children[selectedIndex] as HTMLElement;
    if (item) {
      item.scrollIntoView({ block: "nearest" });
    }
  }, [selectedIndex]);

  const reset = useCallback(() => setSelectedIndex(0), []);

  const clamp = useCallback(
    (length: number) => setSelectedIndex((prev) => clampIndex(prev, length)),
    [],
  );

  const moveNext = useCallback(
    (length: number) => setSelectedIndex((i) => (i + 1) % Math.max(length, 1)),
    [],
  );

  const movePrev = useCallback(
    (length: number) => setSelectedIndex((i) => (i - 1 + length) % Math.max(length, 1)),
    [],
  );

  const selectByIndex = useCallback((i: number) => setSelectedIndex(i), []);

  return { selectedIndex, reset, clamp, moveNext, movePrev, selectByIndex };
}
