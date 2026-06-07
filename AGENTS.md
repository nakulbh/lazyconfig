# lazyconfig — Project Instructions

@plan.md

## What This Project Is

A Rust/GPUI desktop app to manage AI coding agent configuration (skills, subagents, MCP servers, rules, hooks) across 21+ agent platforms. Single-window, macOS-first, pure Rust.

## Development Rules

- Read `plan.md` at the start of every session
- Update `plan.md` phase status when completing tasks
- All code goes in `src/`, following the directory structure in plan.md
- Widgets are built from GPUI primitives (`div()` + styling + event handlers)
- No web stack — pure Rust/GPUI
- Run `cargo fmt` before committing
- Use the GPUI Entity system for state management (not globals where avoidable)
- Study Zed's `crates/ui/src/` for widget patterns when stuck

## Current Phase

**Phase 1: Project Scaffold + Core Widgets**

### Done
- Cargo project initialized (`lazyconfig`)
- `.gitignore` created
- `plan.md` created
- `AGENTS.md` created

### In Progress
- Configure Cargo.toml with GPUI dependency
- Create `main.rs` with window + root view

### Next Up
- Build widget library (Button, TextInput, TextArea, Tabs, Badge, FilterBar, SplitView)
- Define theme.rs and actions.rs

## Build Commands

```bash
cargo build          # Debug build
cargo build --release # Release build
cargo run            # Run the app
cargo fmt            # Format code
cargo clippy         # Lint
```

## Platform Notes

- macOS: GPUI uses Metal backend (Metal 3+ required)
- GPUI dependency pin: `gpui` and `gpui_platform` from `https://github.com/zed-industries/zed`
- Edition: Rust 2024
