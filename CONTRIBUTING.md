# Contributing to ClipContex

Thank you for your interest in contributing to **ClipContex**! Your ideas, code, and feedback help make this project better for everyone. This document outlines the process to help you make your contribution smoothly.

---

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Pull Requests](#pull-requests)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Running Tests](#running-tests)
- [Style Guides](#style-guides)
- [License](#license)

---

## Code of Conduct

Be respectful and considerate. By participating in this project, you agree to abide by the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md) or the community's adopted equivalent.

---

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please file an [issue](https://github.com/abdul-kasif/clipcontex/issues/new?labels=bug) and provide as much detail as possible:
- Clear, descriptive title
- Steps to reproduce
- Expected/actual results
- System/OS/browser information
- Supporting logs or screenshots

### Suggesting Enhancements

Open an [issue](https://github.com/abdul-kasif/clipcontex/issues/new?labels=enhancement) and describe:
- The problem your enhancement solves
- Why it’s beneficial
- Potential implementation ideas

### Pull Requests

Contributions via [pull requests](https://github.com/abdul-kasif/clipcontex/pulls) (PRs) are welcome! To make a successful PR:
1. Discuss significant changes in an issue first.
2. Fork the repository and create a feature branch (use a descriptive name).
3. Make your changes with clear, atomic commits.
4. Include tests for new or changed functionality.
5. Ensure your code follows the style guidelines.
6. Rebase your branch from `main` and resolve all conflicts.
7. Ensure all checks/actions pass.
8. Open a pull request, linking any related issues.

---

## Development Setup

**Prerequisites:**
- [Rust](https://www.rust-lang.org/tools/install) (for backend)
- [Node.js & npm](https://nodejs.org/) (for frontend/Svelte)
- [pnpm](https://pnpm.io/) or [yarn](https://yarnpkg.com/) for dependency management

**Getting Started:**
```bash
# Clone the repository
git clone https://github.com/abdul-kasif/clipcontex.git
cd clipcontex

# Backend setup (Rust)
cd backend
cargo build
cargo test

# Frontend setup (Svelte, TypeScript, CSS)
cd ../frontend
npm install # or pnpm install
npm run dev # or the appropriate start command
```

Check relevant README files in each directory for additional instructions.

---

## Coding Standards

- Rust (backend): Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), [rustfmt](https://doc.rust-lang.org/rustfmt/).
- Svelte/TypeScript/CSS (frontend): Use [Prettier](https://prettier.io/) or local config.
- Write clear, concise, and self-documenting code.
- Add/update documentation as needed.

---

## Commit Messages

Use conventional commits if possible:

```
type(scope): Short summary

Body (optional, wrap at 72 characters)
Footer (optional)
```

**Examples:**
- `fix(clipboard): handle empty clipboard gracefully`
- `feat(ui): add search to clipboard history`

---

## Running Tests

- **Rust:** Run `cargo test` in the backend directory.
- **Frontend:** Use `npm test` or equivalent within the frontend directory.

Make sure all tests pass before submitting a PR.

---

## Style Guides

- **Rust:** `cargo fmt` and `cargo clippy` for linting and formatting.
- **Svelte/TypeScript/CSS:** Use [Prettier](https://prettier.io/) (`npx prettier --write .`).
- Consistent naming and file structure.

---

## License

By contributing, you agree that your contributions will be licensed under the same license as the repository (see [LICENSE](LICENSE)).

---

*Thank you for making ClipContex better!*