# Backend Development Guidelines

> Best practices for backend development in this project.

---

## Overview

This directory contains guidelines for backend development in CCometixLine - a Rust TUI application for generating Claude Code status lines. These guidelines document the **actual patterns** used in the codebase.

---

## Guidelines Index

| Guide | Description | Status |
|-------|-------------|--------|
| [Directory Structure](./directory-structure.md) | Module organization and file layout | ✅ Filled |
| [Database Guidelines](./database-guidelines.md) | Data persistence patterns (no DB, uses TOML/JSON) | ✅ Filled |
| [Error Handling](./error-handling.md) | Error types, handling strategies | ✅ Filled |
| [Quality Guidelines](./quality-guidelines.md) | Code standards, forbidden patterns | ✅ Filled |
| [Logging Guidelines](./logging-guidelines.md) | Output conventions, status messages | ✅ Filled |

---

## Quick Reference

### Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust (Edition 2021) |
| CLI Parsing | clap 4.x |
| TUI Framework | ratatui 0.30 |
| Serialization | serde + toml + serde_json |
| HTTP Client | ureq 3.x |
| Terminal | crossterm 0.29 |

### Key Patterns

1. **Segment Trait**: All status line segments implement `Segment` trait
2. **Config Pattern**: `Config::load()` / `Config::save()` with TOML backend
3. **Error Handling**: `Result<T, Box<dyn std::error::Error>>` with graceful fallbacks
4. **Stateless Design**: No database, file-based configuration

### Module Boundaries

```
config/  → Data structures + file I/O
core/    → Business logic (segments, rendering)
ui/      → Presentation (TUI components, themes)
utils/   → Cross-cutting utilities
```

---

## How to Use These Guidelines

For AI assistants and new contributors:

1. **Before writing code**: Read the relevant guideline file
2. **When adding features**: Follow the patterns in existing code
3. **When in doubt**: Check `directory-structure.md` for module locations

---

**Language**: All documentation is written in **English**.
