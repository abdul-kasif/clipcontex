# ClipContex

**Clipboard Management Tool with Context Awareness**

**ClipContex** is a privacy-respecting clipboard management tool that enriches every clip with contextual metadata - window title, app name and smart tags like ```#code```, ```#url```, and ```#browser``` - all processed and stored locally on your device. No Internet. No Telemetry.
<div align="center">
<img src="/assets/screenshots/clipcontex-main.png" alt="Clipcontex Main" width="800" height="600" />
</div>
---

## Features

### Core Features

- **Context-aware clipboard history**: Capture clips and their contextual metadata (app name and window title)
- **100% offline and local-first**: All data stays on you machine, no account, no cloud dependency
- **Smart auto-tagging**: Automatically categories clips by content (```#json```, ```#url```) and source app (```#browser```, ```#editor```)
- **Fuzzy finding**: search clip using app name, window title, content or tags.
- **Privacy**: Ignore sensitive apps (eg. password managers) to prevent accidental capture
- **Global quick picker (unique)**: Floating window for instant searching and pasting clips from history with a customizable keyboard shortcut
- **Avoid duplication**: built-in deduplication system to avoid clutter
- **Auto Cleanup**: Automatically remove clips older that the configured retention period
- **Pin Important Clips**: Protect key items from auto cleanup.
- **System tray integration**: Access and manage clipcontex from the system tray instantly
- **Auto Launch**: Starts silently in the background when the system boots
- **Cross-Platform**: Native experience on Windows and Linux (Wayland)
- **Lightweight & fast**: Built in Rust - uses <20MB RAM

### Demonstration

#### Global Quick Picker
Triggerd by a custom shortcut ( `Ctrl+Shift+V` by default), search and paste anywhere.

<img src="/assets/videos/clipcontex-quickpicker.gif" alt="Quick Picker" width="620" height="480" />

#### Instant Capture & Smart Auto-Tagging
Captures clip instantly with contextual metadata (window title, app name and tags)

<img src="/assets/videos/clipcontex-capture.gif" alt="Clipcontex Capture" width="620" height="480" />

#### Search and Pin Clips
Fuzzy find clip using tags, content, app name and window title, pin important clips and remove clip from history

<img src="/assets/videos/clipcontex-flow.gif" alt="Clipcontex Flow" width="620" height="480" />

#### Customize and configure Quick Picker Shortcut & other feaures
Customize quick picker shortcut, configure clip's retension period, ignreod apps and enable/disable auto launch

<img src="/assets/screenshots/clipcontex-settings.png" alt="Clipcontex Settings" width="620" height="480" />

## Installation
### Pre-built Binary (recommended)
#### Windows

**ClipContex_0.1.0_x64_en-US.msi** [click here](https://github.com/abdul-kasif/clipcontex/releases/download/v0.1.1/ClipContex_0.1.0_x64_en-US.msi)

#### Linux (Wayland)
**ClipContex_0.1.0_x64_en-US.rpm** (coming soon)

### From Source

#### Prerequisites

- Linux (KDE/Wayland only) **or** Windows 10/11
- [Rust toolchain](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) + [pnpm](https://pnpm.io/) / npm / yarn
- [Tauri dependencies](https://tauri.app/start/prerequisites/)
- Git: For cloning the repository
- **For Linux KDE/Wayland:** <br>`kdotool` (`sudo dnf install kdotool`)

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
Final output: `src-tauri/target/release/bundle/`

Visit the [`releases` page](https://github.com/abdul-kasif/clipcontex/releases) for the latest installers.

## Development

### Prerequisites
- Linux (KDE/Wayland only) **or** Windows 10/11
- [Rust toolchain](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) + [pnpm](https://pnpm.io/) / npm / yarn
- [Tauri dependencies](https://tauri.app/start/prerequisites/)
- Git: For cloning the repository
- **For Linux KDE/Wayland:** <br>`kdotool` (`sudo dnf install kdotool`)

### Building
```sh
# clone repository
git clone https://github.com/abdul-kasif/clipcontex
cd clipcontex

# install frontend packages
pnpm install   # or npm install / yarn install

# install backend crates
cd src-tauri
cargo build

# Run all test
cargo test

# Run application
cd ..
pnpm tauri dev # development mode
```

### Running test
```sh
cd src-tauri

# run all test
cargo test

# with output
cargo test -- --nocapture
```

## Project Structure

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
├── static/               # static files
└── README.md
```

## Roadmap

- **v1.x:** Smart clipboard manager, global quick picker, context & tagging, performance, clean UI
- **v2:** Manual tagging, clip categorization, import/export clips data
- **v3+:** clip editing, custom tagging rules

## Contributing
Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guideline.

Before submitting a PR:

1. Ensure all tests pass: `cargo test`
2. Format code: `cargo fmt --all`
3. Check lints: `cargo clippy`
4. Update documentation if needed

## License

MIT License — Free to use, modify, and distribute.

See [`LICENSE`](https://github.com/abdul-kasif/clipcontex/blob/main/LICENSE).

## Acknowledgments

Built with Rust and powered by:

-   [Tauri](https://v2.tauri.app/) – Secure, lightweight framework for building cross-platform desktop apps with web frontend
-   [Tokio](https://tokio.rs/) - Asynchronous runtime powering Tauri’s backend logic and services
-   [Serde](https://serde.rs/) - Efficient, zero-cost serialization and deserialization of Rust data structures
