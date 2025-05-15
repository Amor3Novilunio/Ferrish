# 🦀 Ferrish

**Ferrish** is a fast, customizable, POSIX-style shell written in Rust — blending the scripting power of Bash with the interactivity of Zsh.

Inspired by the flexibility of modern shells and built from the ground up, Ferrish is a personal tool, a systems playground, and a long-term learning project.

---

## Vision

Ferrish is:
- A minimal core shell (REPL) for parsing and executing commands
- A bridge between **scriptability** (Bash) and **interactivity** (Zsh)
- A future extensible system with plugins, themes, and configuration
- A personal learning journey into system programming with Rust

---

## Goals

- Build a working shell prototype from scratch in Rust
- Support common shell behaviors (`cd`, `ls`, pipes, redirection)
- Create a prompt system and basic scripting
- Integrate smart completion, history, and config loading
- Experiment with plugin architecture and shell extensions

---

## Feature Comparison: Bash vs Zsh vs Ferrish

| Feature / Behavior         | **Bash**           | **Zsh**            | **Ferrish** (Goal)                      |
|---------------------------|--------------------|--------------------|-----------------------------------------|
| 🧱 POSIX compliance        | ✅ Yes             | ⚠️ Mostly           | ✅ Yes – core aligned w/ POSIX          |
| 🔁 Scriptability           | ✅ Widely used     | ⚠️ Less common      | ✅ Full support (sh-like scripting)     |
| 🧠 Interactivity features  | Basic              | ✅ Advanced         | ✅ Smart completion, better input UX    |
| 🧠 Command history         | Simple             | ✅ Shared + smart   | ✅ Persistent, searchable                |
| 🎯 Built-in commands       | ✅ Core (`cd`, etc) | ✅ Core + extras     | ✅ Minimal core built-ins first         |
| 🔌 Plugin system           | ❌ Manual config   | ✅ Oh My Zsh        | ✅ Optional extensions, custom scripts  |
| 🎨 Prompt customization    | Good               | ✅ Theme-friendly   | ✅ Dynamic, colorful, config-based      |
| 📦 Config file             | `.bashrc`          | `.zshrc`           | ✅ `~/.ferrishrc` + TOML/YAML support   |
| 🧰 Autocompletion          | Partial             | ✅ Extensive        | ✅ Path + command aware                 |
| 📂 Globbing support        | ✅ Standard         | ✅ Extended         | ✅ Planned (better file matching)       |
| 🖱 Mouse/keyboard UX       | ❌ None             | ❌ Minimal          | ✅ (via Crossterm in terminal mode)     |
| 📜 Extensible scripting    | ✅ Shell scripts    | ✅ Shell scripts    | ✅ Rust plugin API / shell extensions   |
| ⚡ Speed / Performance     | ✅ Solid            | ⚠️ Slower w/ plugins| ✅ Rust-native, fast by default         |


---
## im no expert but jesus christ this project is too ambitious

## Commit Naming Conventions
| Type       | Purpose                                                                     |
| ---------- | --------------------------------------------------------------------------- |
| `feat`     | **New feature** (e.g. REPL, autocomplete, config loading)                   |
| `fix`      | **Bug fix** or incorrect behavior                                           |
| `refactor` | **Code improvement** that doesn’t change behavior (e.g. cleanup, structure) |
| `perf`     | **Performance improvement**                                                 |
| `style`    | **Code style changes** (formatting, spacing, no logic change)               |
| `test`     | **Add or update tests**                                                     |
| `docs`     | **Documentation updates** (README, inline docs, etc.)                       |
| `chore`    | **Project chores** (build system, tooling, dependencies, infra)             |