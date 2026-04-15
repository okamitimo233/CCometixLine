# Backend Development Guidelines

> Best practices for backend development in this project.

---

## Overview

This directory contains guidelines for backend development in CCometixLine, a Rust CLI tool for generating Claude Code status lines.

---

## Guidelines Index

| Guide | Description | Status |
|-------|-------------|--------|
| [Directory Structure](./directory-structure.md) | Module organization and file layout | Filled |
| [Database Guidelines](./database-guidelines.md) | File-based persistence patterns (no database) | Filled |
| [Error Handling](./error-handling.md) | Error types, handling strategies | Filled |
| [Quality Guidelines](./quality-guidelines.md) | Code standards, forbidden patterns | Filled |
| [Logging Guidelines](./logging-guidelines.md) | Output patterns, stdout/stderr usage | Filled |

---

## Project Summary

**CCometixLine (ccline)** is a high-performance CLI status line generator written in Rust:

- **Architecture**: Modular Rust with clear separation (config/core/ui/utils)
- **Configuration**: TOML files in `~/.claude/ccline/`
- **Output**: Status line to stdout, messages to stderr
- **Patterns**: Result-based error handling, Segment trait for extensibility

### Key Technical Decisions

1. **No database** - File-based configuration and JSON caching
2. **Dynamic errors** - `Box<dyn std::error::Error>` for flexibility
3. **Trait-based segments** - `Segment` trait for consistent segment implementation
4. **Builder pattern** - Fluent API for segment configuration

---

## Quick Reference

### Project Structure
```
src/
├── main.rs, lib.rs, cli.rs, updater.rs
├── config/      # Configuration types and loading
├── core/        # Business logic (segments, statusline generation)
├── ui/          # TUI configurator (ratatui-based)
└── utils/       # Utilities (credentials, patcher)
```

### Error Handling Pattern
```rust
// Use unwrap_or_else for defaults
let config = Config::load().unwrap_or_else(|_| Config::default());

// Use ? for propagation
pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> { ... }
```

### Adding a New Segment
1. Create `src/core/segments/new_segment.rs`
2. Implement `Segment` trait
3. Add to `mod.rs` re-exports
4. Add `SegmentId` variant
5. Register in `collect_all_segments()`

---

**Language**: All documentation is written in **English**.
