# Design Philosophy

## Why fclip exists

At home I use Arch Linux. At work I'm stuck on Windows — but I spend most of my time in WSL anyway.
The problem is when I have to leave WSL and do something on the Windows side.
I need something I copied a while ago, and it's gone.

Clibor was the obvious choice, but every time I opened it, I reached for the mouse.
List, scroll, click. For someone used to fzf, that felt unbearably slow.

On Linux, this is a solved problem.
cliphist + rofi, greenclip + fzf — type a few chars, hit Enter, done.
On Windows, nothing like that existed.

So I built fclip: an fzf-style clipboard manager for Windows.

## Who this is for

Developers who:

- Live primarily in WSL or a terminal, and only occasionally switch to Windows
- Know fzf and expect the same interaction pattern everywhere
- Want clipboard history without a heavy, always-visible app

For example:

- You ran an AWS CLI command 20 minutes ago and need the ARN again
- You're bouncing between files and keep re-copying the same code snippet
- You pasted a Terraform variable in one window and need it in another

fclip is not for everyone. If you want image support, OCR, AI transforms, or a rich GUI, use something else.

## Design decisions

### Text only

If you're copying images, you're doing mouse work — a clipboard manager can't help you there.
fclip handles text and ignores everything else. Content over 100KB is skipped.

### Keyboard only

No mouse interaction is required at any point.
Hotkey to open, type to search, Ctrl+n/p to navigate, Enter to select, Escape to close.
Enter puts the selected item into the clipboard — it does not paste. You paste with Ctrl+V yourself.
The keybindings follow fzf conventions so there's nothing new to learn.

### Windows only

Linux and macOS already have good solutions for this (rofi, fzf, Alfred, Raycast).
Wayland clipboard APIs vary by compositor and aren't worth the effort.
fclip targets Windows because that's where the gap is.

### Minimal by design

Every feature request should be measured against the question:
"Does this help someone fuzzy-search their clipboard history and select it?"

If the answer is no, it doesn't belong in fclip.
The goal is not to compete on features with Ditto or Beetroot.
The goal is to do one thing well — fast, keyboard-driven clipboard recall.

### Configurable, not opinionated

Default keybindings follow fzf conventions, but everything can be changed in `config.toml`.
fclip doesn't force a workflow — it provides sensible defaults and gets out of the way.
