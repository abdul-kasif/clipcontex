<h1 align="center"><strong>ClipContex</strong></h1>

A smart, lightweight clipboard that remembers **what** you were doing, not just **what** you copied.

---

## About

ClipContex is a local-first, privacy-focused clipboard manager for Linux, built for developers and power users. It captures clipboard history along with deep context — including source application, window title, and project — and automatically categorizes your clips using smart tags.

Everything is fast, efficient, and done fully offline; no telemetry, no cloud, no analytics. ClipContex just works in the background, ready when you need it.

> **Note:**  
> ClipContex currently supports Linux on Wayland-based desktop environments, with primary testing and optimization on KDE (Plasma Desktop).
> Support for Windows and additional Linux environments is planned for upcoming releases.

---

## Core Philosophy

1. **Local-First:** Your clipboard content stays on your machine. Forever.
2. **Context Makes Memory Useful:** The clipboard should understand your workflow, not just your texts.
3. **Performance:** A clipboard manager shouldn't be heavier than the apps you use.

---

## Features

- Quick Picker: <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>V</kbd>
- Real-time clipboard saving (300ms debounce)
- Context detection: app, window, project
- Auto-tagging (`#code`, `#url`, `#email`, `#terminal`, `#project`, etc.)
- Pin clips, timeline view, fuzzy search
- Auto-clean (keep history for X days / items)
- Ignores password managers (e.g., Bitwarden, 1Password)
- Memory-optimized (jemalloc)
- Fast native backend (Rust + Tauri)
- No telemetry, no analytics, entirely offline

---

## Technology Stack

| Layer     | Tech                           |
| --------- | ------------------------------ |
| Frontend  | SvelteKit                      |
| Backend   | Rust                           |
| Framework | Tauri                          |
| Database  | SQLite (local-only)            |
| Clipboard | Tauri plugin-clipboard-manager |
| Platform  | Linux (KDE/Wayland only)       |

---

## Screenshots

_(Add your screenshots in `/assets/screenshots/` and link them here)_

| Quick Picker                             | Clip Timeline                        |
| ---------------------------------------- | ------------------------------------ |
| ![](assets/screenshots/quick-picker.png) | ![](assets/screenshots/timeline.png) |

---

## Installation

**Prerequisites**

- Linux (KDE/Wayland only; not for Windows/macOS/X11)
- Rust toolchain (stable)
- Node.js + pnpm/npm/yarn
- [Tauri dependencies](https://tauri.app/start/prerequisites/)

**Clone and Install**

```sh
git clone https://github.com/abdul-kasif/clipcontex
cd clipcontex
pnpm install   # or npm install / yarn install
```

**Development Mode**

```sh
pnpm tauri dev
```

**Build for Production**

```sh
pnpm tauri build
```

Final output: `src-tauri/target/release/`

---

## Directory Overview

```
clipcontex/
├── src/                  # Svelte frontend
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── clipboard/    # Watcher, deduplication, context, active window caching
│   │   ├── config/       # Config handling
│   │   ├── storage/      # SQLite storage
│   │   ├── context/      # Detection of app/window/project
│   │   ├── commands.rs   # Tauri API
│   │   └── lib.rs        # Main setup logic
├── assets/               # Images, icons
└── README.md
```

---

## Contributing

Spotted a bug or got ideas?  
Your feedback, issues, and pull requests are welcome!

**Prerequisites**

- Linux (KDE/Wayland only; not for Windows/macOS/X11)
- Rust toolchain (stable)
- Node.js + pnpm/npm/yarn
- [Tauri dependencies](https://tauri.app/start/prerequisites/)

**Clone and Install**

```sh
git clone https://github.com/abdul-kasif/clipcontex
cd clipcontex
pnpm install   # or npm install / yarn install
```

**Development Mode**

```sh
pnpm tauri dev
```

**Build for Production**

```sh
pnpm tauri build
```

Final output: `src-tauri/target/release/`

**Guidelines**

- Run `rustfmt` on Rust sources before committing
- Follow Svelte code style set in the project
- Strictly no telemetry or user tracking

---

## Roadmap

- **v1.x (Current):** Smart clipboard, fast picker, context & tagging, performance, and clean UI.
- **v2:** Snippets (with placeholders), manual collections, clip editing, configuration GUI.
- **v3+:** Plugin system, custom tagging rules, import/export.

---

## License

MIT License — free to use, modify, and distribute.

---

## Author

Abdul Kasif  
[github.com/abdul-kasif](https://github.com/abdul-kasif)

---

_If ClipContex helps, please consider starring the repo on GitHub to support further development!_

