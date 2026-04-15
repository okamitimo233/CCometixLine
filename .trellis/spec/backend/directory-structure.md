# Directory Structure

> How backend code is organized in this project.

---

## Overview

This is a Rust CLI application for generating Claude Code status lines. The project follows a modular architecture with clear separation between configuration, core business logic, UI components, and utilities.

Key organizational principles:
- Each module has its own directory with a `mod.rs` entry point
- Re-exports are used at module level to simplify public APIs
- Feature-based organization under `core/segments/` for segment implementations
- UI components are isolated in `ui/components/`

---

## Directory Layout

```
src/
├── main.rs              # Application entry point, CLI parsing, main orchestration
├── lib.rs               # Library root, re-exports all public modules
├── cli.rs               # CLI argument definitions (clap-based)
├── updater.rs           # Update checking and state management
│
├── config/              # Configuration loading and type definitions
│   ├── mod.rs           # Module entry point, re-exports
│   ├── loader.rs        # Config file I/O, theme initialization
│   ├── types.rs         # Core data structures (Config, SegmentConfig, etc.)
│   ├── models.rs        # Additional model definitions
│   └── defaults.rs      # Default configuration values
│
├── core/                # Core business logic
│   ├── mod.rs           # Module entry point
│   ├── statusline.rs    # StatusLine generation, segment orchestration
│   └── segments/        # Individual segment implementations
│       ├── mod.rs       # Segment trait definition, re-exports
│       ├── git.rs       # Git segment (branch, status, ahead/behind)
│       ├── model.rs     # Model segment (current AI model)
│       ├── directory.rs # Directory segment (current working directory)
│       ├── usage.rs     # API usage segment (5h/7d utilization)
│       ├── cost.rs      # Cost segment (token costs)
│       ├── context_window.rs # Context window segment
│       ├── session.rs   # Session segment
│       ├── output_style.rs   # Output style segment
│       └── update.rs    # Update notification segment
│
├── ui/                  # Terminal UI (TUI configurator)
│   ├── mod.rs           # Module entry point, run_configurator()
│   ├── app.rs           # Main TUI application state and event loop
│   ├── events.rs        # Event handling
│   ├── layout.rs        # Layout utilities
│   ├── main_menu.rs     # Main menu component
│   ├── components/      # Individual UI components
│   │   ├── mod.rs       # Re-exports all components
│   │   ├── color_picker.rs    # Color selection popup
│   │   ├── editor.rs          # Generic editor component
│   │   ├── help.rs            # Help/instructions display
│   │   ├── icon_selector.rs   # Icon selection popup
│   │   ├── name_input.rs      # Text input for naming
│   │   ├── preview.rs         # Statusline preview
│   │   ├── segment_list.rs    # Segment list panel
│   │   ├── separator_editor.rs # Separator editing
│   │   ├── settings.rs        # Settings panel
│   │   └── theme_selector.rs  # Theme selection
│   └── themes/          # Theme definitions
│       ├── mod.rs       # Theme module entry
│       ├── presets.rs   # Theme preset loader
│       ├── theme_*.rs   # Individual theme files
│
└── utils/               # Utility modules
    ├── mod.rs           # Module entry point
    ├── credentials.rs   # OAuth token retrieval
    └── claude_code_patcher.rs # Claude Code CLI patcher (AST-based)
```

---

## Module Organization

### Entry Points

- `main.rs` - Single entry point for the binary
- `lib.rs` - Library root that re-exports all public modules

Example from `src/lib.rs`:
```rust
pub mod cli;
pub mod config;
pub mod core;
pub mod ui;
pub mod updater;
pub mod utils;
```

### Module Pattern

Each module follows this pattern:

1. **mod.rs** - Declares submodules and re-exports public items
2. **Private implementations** - Internal functions/types in separate files
3. **Public re-exports** - Simplifies imports for consumers

Example from `src/config/mod.rs`:
```rust
pub mod defaults;
pub mod loader;
pub mod models;
pub mod types;

pub use loader::{ConfigLoader, InitResult};
pub use models::*;
pub use types::*;
```

Example from `src/core/segments/mod.rs`:
```rust
pub trait Segment {
    fn collect(&self, input: &InputData) -> Option<SegmentData>;
    fn id(&self) -> SegmentId;
}

// Re-export all segment types
pub use context_window::ContextWindowSegment;
pub use cost::CostSegment;
pub use directory::DirectorySegment;
pub use git::GitSegment;
// ... more re-exports
```

### Adding New Segments

1. Create new file in `src/core/segments/` (e.g., `new_feature.rs`)
2. Implement the `Segment` trait
3. Add module declaration and re-export in `mod.rs`
4. Add new `SegmentId` variant in `src/config/types.rs`
5. Register segment in `collect_all_segments()` in `statusline.rs`

---

## Naming Conventions

### File Names

- Use `snake_case` for file names: `claude_code_patcher.rs`, `color_picker.rs`
- Module directories match module names: `config/`, `core/`, `ui/`

### Type Names

- Use `PascalCase` for struct/enum names: `Config`, `SegmentConfig`, `StatusLineGenerator`
- Use `PascalCase` for enum variants: `StyleMode::Plain`, `GitStatus::Clean`

### Function Names

- Use `snake_case` for function names: `get_git_info()`, `collect_all_segments()`
- Private helper functions prefixed with context: `find_spinner_verbose_in_node()`

### Variable Names

- Use `snake_case` for variables: `working_dir`, `segment_config`
- Short names acceptable in small scopes: `s` for string slices, `v` for values

---

## Examples

### Well-Organized Module: `core/segments/`

The segments module demonstrates good organization:

1. **Clear trait definition** (`mod.rs`):
```rust
pub trait Segment {
    fn collect(&self, input: &InputData) -> Option<SegmentData>;
    fn id(&self) -> SegmentId;
}
```

2. **Consistent implementation pattern** (each segment file):
```rust
// git.rs
pub struct GitSegment {
    show_sha: bool,
}

impl Segment for GitSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> { ... }
    fn id(&self) -> SegmentId { SegmentId::Git }
}
```

3. **Central orchestration** (`statusline.rs`):
```rust
pub fn collect_all_segments(config: &Config, input: &InputData)
    -> Vec<(SegmentConfig, SegmentData)> { ... }
```

### Configuration Pattern: `config/`

1. **Types isolated** (`types.rs`) - All data structures
2. **Loading logic** (`loader.rs`) - File I/O operations
3. **Module re-exports** (`mod.rs`) - Clean public API
