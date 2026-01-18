<!--- GitHub Action Banner, Star & Fork --->
<p align="center">
  <a href="https://github.com/abdul-kasif/clipcontex/fork"><img src="https://img.shields.io/github/forks/abdul-kasif/clipcontex?style=social" alt="Fork"></a>
  <a href="https://github.com/abdul-kasif/clipcontex/stargazers"><img src="https://img.shields.io/github/stars/abdul-kasif/clipcontex?style=social" alt="Star"></a>
  <span>
    <img src="https://img.shields.io/badge/Rust-57.8%25-blueviolet?logo=rust">
    <img src="https://img.shields.io/badge/Svelte-35.2%25-ff3e00?logo=svelte">
    <img src="https://img.shields.io/badge/TypeScript-5%25-3178c6?logo=typescript">
    <img src="https://img.shields.io/badge/CSS-1.1%25-2965f1?logo=css3">
    <img src="https://img.shields.io/badge/Other-0.9%25-lightgrey">
  </span>
</p>

<h1 align="center">
  <img src="static/32x32.png" alt="ClipContex logo" height="32" style="vertical-align:middle; margin-bottom:-8px;" />
  <strong>ClipContex</strong>
</h1>

<!-- Hero Banner -->
<p align="center" style="margin-bottom:-16px;">
  <b>Instant context. Instant productivity.</b>
</p>
<p align="center">
  <video src="assets/videos/clipcontex-demo.mp4" width="640" controls poster="assets/screenshots/clipcontex-poster.png"></video>
</p>
<p align="center"><b>Your clipboard should remember what you were doing, not just what you copied.</b></p>

---

## 🚀 One-Line Pitch
**ClipContex** is a smart, lightning-fast clipboard manager for Linux and Windows, built for developers and power users. It doesn’t just track copy history—it remembers the context, app, and project, organizes everything with smart tags, and works fully offline.

---

## 💡 Features

<div align="center">

<table>
  <tr>
    <td align="center"><span title="Quick Picker">⚡</span><br><b>Quick Picker</b><br><sub>Summon with <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>V</kbd></sub></td>
    <td align="center"><span title="Context Detection">🧠</span><br><b>Context Awareness</b><br><sub>Knows app, window, and project</sub></td>
    <td align="center"><span title="Auto-Tagging">🏷️</span><br><b>Smart Tagging</b><br><sub>#code #url #email #terminal</sub></td>
    <td align="center"><span title="Search & Timeline">🔎</span><br><b>Fuzzy Search & Timeline</b><br><sub>Always find your past work</sub></td>
  </tr>
  <tr>
    <td align="center"><span title="Pin">📌</span><br><b>Pin Clips</b><br><sub>Keep important items forever</sub></td>
    <td align="center"><span title="Auto-Clean">🧹</span><br><b>Auto-Clean</b><br><sub>Clean old history automatically</sub></td>
    <td align="center"><span title="Privacy">🔐</span><br><b>100% Offline</b><br><sub>No cloud, no telemetry, no analytics</sub></td>
    <td align="center"><span title="Performance">🚀</span><br><b>Ultra-Fast & Lightweight</b><br><sub>Rust + Tauri, memory optimized</sub></td>
  </tr>
  <tr>
    <td align="center"><span title="Native">🖥️</span><br><b>Native Backend</b><br><sub>Integrates with Linux & Windows</sub></td>
    <td align="center"><span title="Password Managers">🙈</span><br><b>Ignores Password Managers</b><br><sub>Bitwarden, 1Password, etc.</sub></td>
    <td align="center"><span title="Customizable">🛠️</span><br><b>Configurable</b><br><sub>Auto-start, hotkeys, & more</sub></td>
    <td align="center"><span title="Clipboard Types">🗂️</span><br><b>Text Only</b><br><sub>Clipboard privacy by design</sub></td>
  </tr>
</table>

</div>
<br>

---

## 🔗 Installation

**👉 [See the releases for downloads](https://github.com/abdul-kasif/clipcontex/releases)**

### Linux (KDE/Wayland only) & Windows

#### Prerequisites

- Linux (KDE/Wayland only) **or** Windows 10/11
- [Rust toolchain](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) + [pnpm](https://pnpm.io/) / npm / yarn
- [Tauri dependencies](https://tauri.app/start/prerequisites/)
- **For Linux KDE/Wayland:** <br>`kdotool` (`sudo apt install kdotool`)

#### Clone and Install

```sh
git clone https://github.com/abdul-kasif/clipcontex
cd clipcontex
pnpm install   # or npm install / yarn install
```

#### Development Mode
```sh
pnpm tauri dev
```

#### Build for Production
```sh
pnpm tauri build
```

Final output: `src-tauri/target/release/`

Visit the [`releases` page](https://github.com/abdul-kasif/clipcontex/releases) for the latest installers.

---

## 🏗️ Tech Stack

| Layer     | Tech                           |
| --------- | ------------------------------ |
| Frontend  | SvelteKit                      |
| Backend   | Rust                           |
| Framework | Tauri                          |
| Database  | SQLite (local-only)            |
| Clipboard | Tauri plugin-clipboard-manager |
| Platform  | Linux (KDE/Wayland), Windows   |

---

## 🗂️ Project Structure

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
├── assets/               # Images, icons, screenshots
├── static/               # Demo videos and static files
└── README.md
```

---

## 🤝 Contributing

<p>
  <span style="font-size:1.5em;vertical-align:middle;" title="Contributing">🌟</span> 
  <b>We welcome all feedback and contributions!</b> 
</p>

- Spot a bug or have an idea? All issues & PRs are welcome!  
- Start a discussion or check [issues](https://github.com/abdul-kasif/clipcontex/issues)
- Check our <a href="https://github.com/abdul-kasif/clipcontex/blob/main/CONTRIBUTING.md"><b>CONTRIBUTING.md</b></a> for details and guidelines.

**Contribution Standards**
- Run `rustfmt` on Rust sources before committing
- Follow Svelte code style set in the project
- **No telemetry, tracking, or analytics — privacy is design**
- [Open collective feedback/PRs](https://github.com/abdul-kasif/clipcontex/pulls) appreciated!

Want to help? Star ⭐ the repo or [fork](https://github.com/abdul-kasif/clipcontex/fork) and contribute!

---

## 📅 Roadmap

- **v1.x:** Smart clipboard, fast picker, context & tagging, performance, clean UI
- **v2:** Snippets with placeholders, manual collections, clip editing, configuration GUI
- **v3+:** Plugin system, custom tagging rules, import/export

---

## 📜 License

MIT License — Free to use, modify, and distribute.

See [`LICENSE`](https://github.com/abdul-kasif/clipcontex/blob/main/LICENSE).

---

## 👤 Author

**Abdul Kasif**  
[github.com/abdul-kasif](https://github.com/abdul-kasif)

_If ClipContex helps, please consider starring the repo to support further development!_

---