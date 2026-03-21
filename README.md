# fclip

A fast, keyboard-driven clipboard manager for Windows built with Tauri (Rust + React).
Designed to replace bloated clipboard tools with a minimal, fzf-style experience.

## Overview

fclip runs in the system tray and silently monitors your clipboard.
Press a hotkey to instantly summon a fuzzy-search popup — type a few characters,
hit Enter, and the selected item is copied to your clipboard. No mouse required.

## Features

- Fuzzy search across full clipboard history (fzf algorithm)
- Keyboard-only workflow (hotkey → search → Enter to select)
- Pin entries with labels to keep them from being pruned
- Configurable hotkey and keybindings
- Persistent history across restarts (SQLite)
- Event-driven clipboard monitoring (no polling)
- Skips clipboard content larger than 100KB
- System tray with double-click to toggle

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React + TypeScript (Vite) |
| Backend | Rust (Tauri v2) |
| Clipboard monitoring | `clipboard-master` (WM_CLIPBOARDUPDATE events) |
| Clipboard read/write | `clipboard-win` |
| Fuzzy search | `fuzzy-matcher` |
| Storage | SQLite via `rusqlite` (WAL mode, optimized PRAGMAs) |
| Config | TOML (`%APPDATA%\fclip\config.toml`) |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Development

```bash
npm install
npm run dev
```

### Build

```bash
npm run build
```

### Other Commands

```bash
npm test          # Run Rust tests
npm run check     # Compile check only
npm run lint      # Run clippy + tsc
npm run clean     # Remove build artifacts
```

## Usage

1. fclip starts in the system tray
2. Press `Ctrl+Shift+V` (default) to open the search window
3. Type to fuzzy-search your clipboard history
4. Press `Enter` to copy the selected item to clipboard
5. Double-click the tray icon to toggle the window

### Default Keybindings

| Key | Action |
|-----|--------|
| `Enter` | Select item (copy to clipboard) |
| `Escape` | Close window |
| `Ctrl+n` | Next item |
| `Ctrl+p` | Previous item |
| `Ctrl+d` | Delete item |
| `Ctrl+f` | Pin/unpin item (with label) |
| `Ctrl+h` | Backspace in search |

## Configuration

Create `%APPDATA%\fclip\config.toml` to customize (see `examples/config.toml`):

```toml
[hotkey]
open = "Ctrl+Shift+V"

[behavior]
max_history = 1000

[keybindings]
select = "Enter"
close = "Escape"
delete = "Ctrl+d"
next = "Ctrl+n"
prev = "Ctrl+p"
```

## Architecture

```
OS Clipboard
    ↓ WM_CLIPBOARDUPDATE event
Rust backend — clipboard-master → dedup (SHA-256) → SQLite (WAL)
    ↓ Tauri IPC
React frontend — fuzzy-matcher → ranked list → select on Enter
```

## License

MIT
