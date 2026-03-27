# fclip

A fast, keyboard-driven clipboard manager for Windows built with Tauri (Rust + React).
Designed to replace bloated clipboard tools with a minimal, fzf-style experience.

## Overview

fclip runs in the system tray and silently monitors your clipboard.
Press a hotkey to instantly summon a fuzzy-search popup — type a few characters,
hit Enter, and the selected item is copied to your clipboard. No mouse required.

## Tech Stack

- **Frontend**: React + TypeScript (Vite)
- **Backend**: Rust (Tauri v2)
- **Clipboard monitoring**: `clipboard-master` crate (event-driven via WM_CLIPBOARDUPDATE)
- **Clipboard read/write**: `clipboard-win` crate
- **Fuzzy search**: `fuzzy-matcher` crate (fzf algorithm)
- **Storage**: SQLite via `rusqlite` (WAL mode, optimized PRAGMAs)
- **Config**: TOML at `%APPDATA%\fclip\config.toml`

## Key Features

- Fuzzy search across full clipboard history
- Keyboard-only workflow (hotkey → search → Enter to select)
- Pin entries with labels (protected from history limit)
- Configurable hotkey (default: Ctrl+Shift+V)
- Light/dark theme support (with OS theme detection)
- Persistent history across restarts
- Windows-specialized (event-driven monitoring, NSIS/MSI installer)
- Skips clipboard content larger than 100KB

## Configuration

```toml
[hotkey]
open = "Ctrl+Shift+V"

[behavior]
max_history = 1000

[theme]
mode = "system"              # "dark", "light", or "system" (follows OS)

[keybindings]
select = "Enter"
close = "Escape"
delete = "Ctrl+d"
next = "Ctrl+n,Ctrl+j"      # comma-separated for multiple bindings
prev = "Ctrl+p,Ctrl+k"
backspace = "Ctrl+h"
clear = "Ctrl+u"
toggle_theme = "Ctrl+t"
help = "Ctrl+?"
```

## Development

```bash
npm install
npm run dev
```

## Architecture

OS Clipboard
    ↓ WM_CLIPBOARDUPDATE event
Rust backend — clipboard-master → dedup → SQLite
    ↓ Tauri IPC
React frontend — fuzzy-matcher → ranked list → select on Enter
