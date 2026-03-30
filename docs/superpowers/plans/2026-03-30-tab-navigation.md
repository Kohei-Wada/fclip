# Tab Navigation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add All/Pin tab navigation so pinned items are displayed on a separate tab, reducing clutter in the main clipboard history view.

**Architecture:** Frontend-only filtering. The existing `search_clipboard` IPC returns all items. A new `activeTab` state in App determines which subset to display. New `tab_next`/`tab_prev` keybindings are added to Rust config and TypeScript types.

**Tech Stack:** React (frontend state + filtering), Rust (keybinding config), CSS (tab bar styling)

---

### Task 1: Add `tab_next`/`tab_prev` to Rust keybinding config

**Files:**
- Modify: `src-tauri/src/config.rs:36-57` (KeybindingsConfig struct)
- Modify: `src-tauri/src/config.rs:74-76` (default_next/default_prev functions)
- Modify: `src-tauri/src/config.rs:132-144` (KeybindingsResponse struct)
- Modify: `src-tauri/src/config.rs:170-184` (to_response method)
- Modify: `src-tauri/src/config.rs:187-202` (Default impl)

- [ ] **Step 1: Add failing tests for new keybindings**

Add these tests at the end of the `mod tests` block in `src-tauri/src/config.rs`:

```rust
#[test]
fn test_default_tab_keybindings() {
    let config = Config::default();
    assert_eq!(config.keybindings.tab_next, "Ctrl+n");
    assert_eq!(config.keybindings.tab_prev, "Ctrl+p");
    // next/prev defaults should no longer include Ctrl+n/Ctrl+p
    assert_eq!(config.keybindings.next, "Ctrl+j");
    assert_eq!(config.keybindings.prev, "Ctrl+k");
}

#[test]
fn test_tab_keybindings_response() {
    let config = Config::default();
    let resp = config.keybindings.to_response();
    assert_eq!(resp.tab_next.len(), 1);
    assert_eq!(resp.tab_next[0].key, "n");
    assert!(resp.tab_next[0].ctrl);
    assert_eq!(resp.tab_prev.len(), 1);
    assert_eq!(resp.tab_prev[0].key, "p");
    assert!(resp.tab_prev[0].ctrl);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd src-tauri && cargo test`
Expected: compilation errors — `tab_next`, `tab_prev` fields don't exist yet.

- [ ] **Step 3: Add default functions for tab_next and tab_prev**

Add after `default_open_config` (line 93):

```rust
fn default_tab_next() -> String {
    "Ctrl+n".to_string()
}
fn default_tab_prev() -> String {
    "Ctrl+p".to_string()
}
```

- [ ] **Step 4: Change default_next and default_prev**

Change the existing functions:

```rust
fn default_next() -> String {
    "Ctrl+j".to_string()
}
fn default_prev() -> String {
    "Ctrl+k".to_string()
}
```

- [ ] **Step 5: Add fields to KeybindingsConfig**

Add to the `KeybindingsConfig` struct (after `open_config` field):

```rust
#[serde(default = "default_tab_next")]
pub tab_next: String,
#[serde(default = "default_tab_prev")]
pub tab_prev: String,
```

- [ ] **Step 6: Add fields to KeybindingsResponse**

Add to the `KeybindingsResponse` struct (after `open_config` field):

```rust
pub tab_next: Vec<Key>,
pub tab_prev: Vec<Key>,
```

- [ ] **Step 7: Add to to_response method**

Add to the `to_response` method body (after `open_config` line):

```rust
tab_next: parse_bindings(&self.tab_next),
tab_prev: parse_bindings(&self.tab_prev),
```

- [ ] **Step 8: Add to Default impl**

Add to the `Default` impl for `KeybindingsConfig` (after `open_config`):

```rust
tab_next: default_tab_next(),
tab_prev: default_tab_prev(),
```

- [ ] **Step 9: Update existing test assertions**

In `test_default_config`, update:
```rust
assert_eq!(config.keybindings.next, "Ctrl+j");
assert_eq!(config.keybindings.prev, "Ctrl+k");
```

In `test_keybindings_response_default_values`, update:
```rust
assert_eq!(resp.next.len(), 1);
assert_eq!(resp.next[0].key, "j");
assert!(resp.next[0].ctrl);
assert_eq!(resp.prev.len(), 1);
assert_eq!(resp.prev[0].key, "k");
assert!(resp.prev[0].ctrl);
```

In `test_keybindings_response_multiple_values`, update the assertions to match the new defaults (the test uses custom config so `next`/`prev` stay as `Ctrl+n,Ctrl+j` / `Ctrl+p,Ctrl+k` from the TOML — those assertions are fine).

- [ ] **Step 10: Run tests to verify they pass**

Run: `cd src-tauri && cargo test`
Expected: all tests pass.

- [ ] **Step 11: Commit**

```bash
git add src-tauri/src/config.rs
git commit -m "feat: add tab_next/tab_prev keybindings, change next/prev defaults"
```

---

### Task 2: Add `tab_next`/`tab_prev` to TypeScript types

**Files:**
- Modify: `src/types.ts:30-41` (Keybindings interface)

- [ ] **Step 1: Add fields to Keybindings interface**

Add after `open_config: Key[];` in `src/types.ts`:

```typescript
tab_next: Key[];
tab_prev: Key[];
```

- [ ] **Step 2: Commit**

```bash
git add src/types.ts
git commit -m "feat: add tab_next/tab_prev to Keybindings type"
```

---

### Task 3: Create TabBar component

**Files:**
- Create: `src/components/TabBar.tsx`
- Modify: `src/App.css` (add tab bar styles)

- [ ] **Step 1: Create TabBar component**

Create `src/components/TabBar.tsx`:

```tsx
export type Tab = "all" | "pin";

export function TabBar({ activeTab }: { activeTab: Tab }) {
  return (
    <div className="tab-bar">
      <span className={`tab-item ${activeTab === "all" ? "active" : ""}`}>All</span>
      <span className={`tab-item ${activeTab === "pin" ? "active" : ""}`}>Pin</span>
    </div>
  );
}
```

- [ ] **Step 2: Add tab bar styles to App.css**

Add at the end of `src/App.css`:

```css
.tab-bar {
  display: flex;
  gap: 0;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
  padding: 0 12px;
}

.tab-item {
  padding: 4px 16px;
  color: var(--text-muted);
  font-size: 12px;
  cursor: default;
  border-bottom: 2px solid transparent;
  user-select: none;
}

.tab-item.active {
  color: var(--accent);
  border-bottom-color: var(--accent);
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/TabBar.tsx src/App.css
git commit -m "feat: add TabBar component with styling"
```

---

### Task 4: Integrate tab state and filtering into App

**Files:**
- Modify: `src/App.tsx`

- [ ] **Step 1: Add imports and tab state**

Add import at top of `src/App.tsx`:

```typescript
import { TabBar, type Tab } from "./components/TabBar";
```

Add state after `showHelp` state (line 29):

```typescript
const [activeTab, setActiveTab] = useState<Tab>("all");
```

- [ ] **Step 2: Add filtered results**

Add after the `activeTab` state, a derived value:

```typescript
const filteredResults = activeTab === "all"
  ? results.filter((r) => !r.pinned)
  : results.filter((r) => r.pinned);
```

- [ ] **Step 3: Add tab switching keybinding handlers**

Add in the `handleKeyDown` function, after the `showHelp` block (after line 106) and before the `Ctrl+[` block:

```typescript
if (matchesKeybinding(e, keybindings.tab_next)) {
  e.preventDefault();
  setActiveTab((t) => (t === "all" ? "pin" : "all"));
  cursor.reset();
  return;
}

if (matchesKeybinding(e, keybindings.tab_prev)) {
  e.preventDefault();
  setActiveTab((t) => (t === "pin" ? "all" : "pin"));
  cursor.reset();
  return;
}
```

- [ ] **Step 4: Add TabBar to JSX and use filteredResults**

Add `<TabBar>` in the JSX, after the search bar section (after line 198, before `<div className="results-container">`):

```tsx
<TabBar activeTab={activeTab} />
```

Replace all references to `results` in the JSX and handlers with `filteredResults`:
- Line 128: `cursor.moveNext(results.length)` → `cursor.moveNext(filteredResults.length)`
- Line 131: `cursor.movePrev(results.length)` → `cursor.movePrev(filteredResults.length)`
- Line 134: `results[cursor.selectedIndex]` → `filteredResults[cursor.selectedIndex]`
- Line 135: `results[cursor.selectedIndex].id` → `filteredResults[cursor.selectedIndex].id`
- Line 141: `results[cursor.selectedIndex]` → `filteredResults[cursor.selectedIndex]`
- Line 141: `results[cursor.selectedIndex].pinned` → `filteredResults[cursor.selectedIndex].pinned`
- Line 143: `results[cursor.selectedIndex].id` → `filteredResults[cursor.selectedIndex].id`
- Line 154: `results[cursor.selectedIndex]` → `filteredResults[cursor.selectedIndex]`
- Line 156: `results[cursor.selectedIndex]` → `filteredResults[cursor.selectedIndex]`
- Line 195: `resultCount={results.length}` → `resultCount={filteredResults.length}`
- Line 201: `results={results}` → `results={filteredResults}`

- [ ] **Step 5: Reset tab to "all" on window focus**

In `useClipboardSearch.ts`, the window focus handler already re-focuses. But tab reset should happen in App. Add to the existing focus listener concept — add a new effect in App.tsx after the theme effect:

```typescript
useEffect(() => {
  const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
    if (focused) {
      setActiveTab("all");
    }
  });
  return () => { unlisten.then((f) => f()); };
}, []);
```

- [ ] **Step 6: Run dev to verify visually**

Run: `npm run dev`
Verify: Tab bar appears, Ctrl+N/P switches tabs, items are correctly filtered.

- [ ] **Step 7: Commit**

```bash
git add src/App.tsx
git commit -m "feat: integrate tab navigation with filtering in App"
```

---

### Task 5: Update StatusBar and HelpOverlay

**Files:**
- Modify: `src/components/HelpOverlay.tsx:9-25` (buildEntries function)
- Modify: `src/components/StatusBar.tsx:15-19` (status text)

- [ ] **Step 1: Add tab navigation to HelpOverlay**

In `src/components/HelpOverlay.tsx`, add to the `buildEntries` array (after the "toggle help" entry):

```typescript
{ keys: fmt(kb.tab_next), action: "next tab" },
{ keys: fmt(kb.tab_prev), action: "prev tab" },
```

- [ ] **Step 2: Add tab keybinding to StatusBar**

In `src/components/StatusBar.tsx`, update the `text` template to include tab info. Replace the existing text assignment:

```typescript
const text = keybindings
  ? `${fmt(keybindings.select)}: select | ${fmt(keybindings.close)}: close | ${fmt(keybindings.delete)}: delete | ${fmt(keybindings.tab_next)}: tab | ${fmt(keybindings.help)}: help`
  : "";
```

- [ ] **Step 3: Commit**

```bash
git add src/components/HelpOverlay.tsx src/components/StatusBar.tsx
git commit -m "feat: show tab keybindings in StatusBar and HelpOverlay"
```

---

### Task 6: Update example config

**Files:**
- Modify: `examples/config.toml`

- [ ] **Step 1: Update config.toml**

Update the keybindings section in `examples/config.toml`:

```toml
[keybindings]
select = "Enter"
close = "Escape"
delete = "Ctrl+d"
next = "Ctrl+j"
prev = "Ctrl+k"
backspace = "Ctrl+h"
clear = "Ctrl+u"
toggle_theme = "Ctrl+t"
help = "Ctrl+?"
open_config = "Ctrl+e"
tab_next = "Ctrl+n"
tab_prev = "Ctrl+p"
```

- [ ] **Step 2: Commit**

```bash
git add examples/config.toml
git commit -m "docs: update example config with tab keybindings"
```

---

### Task 7: Update CLAUDE.md

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1: Update keybindings section in CLAUDE.md**

Update the `[keybindings]` example in CLAUDE.md to match the new defaults:

```toml
[keybindings]
select = "Enter"
close = "Escape"
delete = "Ctrl+d"
next = "Ctrl+j"
prev = "Ctrl+k"
backspace = "Ctrl+h"
clear = "Ctrl+u"
toggle_theme = "Ctrl+t"
help = "Ctrl+?"
open_config = "Ctrl+e"
tab_next = "Ctrl+n"
tab_prev = "Ctrl+p"
```

- [ ] **Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with tab keybindings"
```
