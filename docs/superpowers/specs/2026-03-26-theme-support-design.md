# Theme Support Design (Light/Dark Mode)

Issue: #1

## Overview

Add light/dark theme support to fclip using CSS custom properties and a `data-theme` attribute on the root element. Users configure the initial theme in `config.toml`; a keyboard shortcut toggles themes at runtime (non-persistent).

## Requirements

- Three theme modes: `dark`, `light`, `system`
- `system` resolves to dark/light based on OS preference at startup (no realtime tracking)
- Runtime toggle via configurable keybinding (default `Ctrl+t`), not persisted to config
- Accent color (#e94560) shared across both themes

## Config

Add `[theme]` section to `config.toml`:

```toml
[theme]
mode = "system"   # "dark" | "light" | "system"
```

Default value: `"system"`.

Add `toggle_theme` to `[keybindings]`:

```toml
[keybindings]
toggle_theme = "Ctrl+t"
```

## Rust Changes

### ThemeConfig struct

New struct in `config.rs`:

```rust
#[derive(Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_mode")]
    pub mode: String,
}

fn default_theme_mode() -> String {
    "system".to_string()
}
```

Add `theme: ThemeConfig` field to `Config` with `#[serde(default)]`.

### KeybindingsConfig

Add `toggle_theme` field with default `"Ctrl+t"`. Include in `KeybindingsResponse`.

### IPC Command

New command `get_theme` returns the mode string:

```rust
#[tauri::command]
pub fn get_theme(state: tauri::State<AppState>) -> String {
    state.config.theme.mode.clone()
}
```

## CSS Changes

### Color tokens

Define CSS custom properties under `data-theme` selectors at the top of `App.css`:

```css
:root[data-theme="dark"] {
  --bg-primary: #1a1a2e;
  --bg-secondary: #16213e;
  --border: #0f3460;
  --text: #e0e0e0;
  --text-muted: #555;
  --accent: #e94560;
  --accent-hover: rgba(233, 69, 96, 0.15);
  --selected-bg: #16213e;
  --scrollbar-thumb: #0f3460;
  --scrollbar-track: #1a1a2e;
}

:root[data-theme="light"] {
  --bg-primary: #ffffff;
  --bg-secondary: #f5f5f5;
  --border: #e0e0e0;
  --text: #333333;
  --text-muted: #999999;
  --accent: #e94560;
  --accent-hover: rgba(233, 69, 96, 0.1);
  --selected-bg: #f0f4ff;
  --scrollbar-thumb: #ccc;
  --scrollbar-track: #ffffff;
}
```

### Replace hardcoded colors

All hardcoded color values in App.css replaced with `var(--token)` references. No structural CSS changes needed.

## Frontend Changes

### Theme initialization (App.tsx)

On mount:
1. `invoke("get_theme")` to get mode string
2. If `"system"`: resolve via `window.matchMedia("(prefers-color-scheme: dark)")`
3. Set `document.documentElement.dataset.theme` to resolved value
4. Store in `useState<"dark" | "light">`

### Toggle keybinding

In `handleKeyDown`, match `toggle_theme` keybinding:
- Flip state between `"dark"` and `"light"`
- Update `document.documentElement.dataset.theme`

### Types

Add `toggle_theme: Key[]` to `Keybindings` interface in `types.ts`.

## Scope Exclusions

- No custom color configuration (users cannot define their own colors)
- No realtime OS theme tracking (startup-only for `system` mode)
- No persistence of runtime toggle (reverts to config on restart)
- No additional themes beyond dark/light
