# Ferrish 🐚

**Ferrish** is a minimal shell written in Rust.

At its core, Ferrish works just like any other shell — it lets you run commands, move around your system, and use the terminal. But unlike other shells, Ferrish doesn’t come with any built-in “smart” features, themes, or extra tools.

It’s just a shell.

## 🧠 Philosophy

Other shells (like Bash, Zsh, or Fish) each have their own unique features, quirks, and built-in behaviors. Ferrish doesn’t.

Instead, **Ferrish stays minimal** — and lets *you* decide what features to add by installing plugins.

> Think of Ferrish as a blank canvas.  
> The core does the basics. Everything else is up to you.

## 🔍 What Makes It Different?

- 🚫 No built-in bloat
- ✅ Just the basics: input, output, command execution
- 🔌 Add features through plugins (coming soon)
- 🛠 Written in Rust for clarity, safety, and performance

## 🚧 Roadmap

- [x] REPL loop (type a command, run it)
- [ ] Command history
- [ ] Basic config system
- [ ] Plugin loader
- [ ] Define plugin structure (e.g. manifest + hooks)
- [ ] Plugin examples (prompt, alias, git info, etc.)