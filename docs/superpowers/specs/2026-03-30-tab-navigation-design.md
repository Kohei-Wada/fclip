# Tab Navigation: All / Pin

## Summary

Add a tab system to separate pinned items from clipboard history. Two tabs: **All** (non-pinned only) and **Pin** (pinned only). Tabs are navigated via configurable keybindings.

## Motivation

When many items are pinned, they crowd the top of the result list and make it hard to find recent clipboard entries. Separating pins into their own tab keeps both views clean and focused.

## Design

### Tabs

| Tab  | Content                        | Default |
|------|--------------------------------|---------|
| All  | Non-pinned clipboard entries   | Yes     |
| Pin  | Pinned entries only            | No      |

- App starts on the All tab.
- Tab state resets to All when the window is re-shown.
- Switching tabs resets cursor to index 0.
- Search query persists across tab switches (filters apply within each tab).

### Keybindings

New configurable keybindings:

```toml
[keybindings]
tab_next = "Ctrl+n"
tab_prev = "Ctrl+p"
```

Existing defaults changed:
- `next`: `Ctrl+j` (was `Ctrl+n,Ctrl+j`)
- `prev`: `Ctrl+k` (was `Ctrl+p,Ctrl+k`)

### Data Flow

No backend changes. Filtering is done on the frontend:

```
search_clipboard (returns all items)
    |
    +-- All tab  -> results.filter(r => !r.pinned)
    +-- Pin tab  -> results.filter(r => r.pinned)
```

### UI

**Tab indicator**: Displayed above the search bar. Minimal text-based indicator showing which tab is active.

```
  All   Pin
  ^^^
  (active tab underlined or highlighted)
```

**StatusBar**: Add tab_next/tab_prev keybinding display.

**HelpOverlay**: Add tab navigation entries.

### Tab indicator component

New component `TabBar` renders the two tab labels. Active tab uses accent color/underline styling. Inactive tab uses muted text.

## Files to Change

| File | Change |
|------|--------|
| `src/App.tsx` | Add tab state, Ctrl+N/P handlers, filter results by tab, reset cursor on tab switch |
| `src/components/TabBar.tsx` | New component: tab indicator UI |
| `src/components/ResultList.tsx` | No change (receives filtered results) |
| `src/hooks/useClipboardSearch.ts` | Export unfiltered results; add filtered view or let App filter |
| `src/types.ts` | Add `tab_next`, `tab_prev` to `Keybindings` interface |
| `src/App.css` | Tab bar styles |
| `src/components/StatusBar.tsx` | Show tab keybindings |
| `src/components/HelpOverlay.tsx` | Add tab navigation help entries |
| `src-tauri/src/config.rs` | Add `tab_next`/`tab_prev` keybinding parsing + defaults, change `next`/`prev` defaults |
| `examples/config.toml` | Add `tab_next`/`tab_prev` entries, update `next`/`prev` |

## Edge Cases

- **No results in active tab**: Show empty state (e.g., "No pinned items" for Pin tab).
- **Pin/unpin while on Pin tab**: Refresh should update the filtered list. Item disappears from Pin tab when unpinned.
- **Delete on Pin tab**: Delete is blocked for pinned items (existing behavior), so no change needed.
