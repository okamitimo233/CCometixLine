# Directory Structure

> How backend code is organized in this project.

---

## Overview

CCometixLine is a Rust TUI application for generating Claude Code status lines. The codebase follows a layered architecture with clear separation between configuration, core logic, and UI.

---

## Directory Layout

```
src/
├── main.rs           # Entry point, CLI handling, stdin processing
├── lib.rs            # Module exports
├── cli.rs            # CLI argument parsing (clap)
├── updater.rs        # Version update checking
│
├── config/           # Configuration layer
│   ├── mod.rs        # Module exports
│   ├── types.rs      # Config structs (Config, SegmentConfig, etc.)
│   ├── loader.rs     # ConfigLoader, file I/O
│   ├── models.rs     # Model-specific config
│   └── defaults.rs   # Default values
│
├── core/             # Business logic layer
│   ├── mod.rs        # Module exports
│   ├── statusline.rs # StatusLineGenerator, segment rendering
│   └── segments/     # Segment implementations
│       ├── mod.rs    # Segment trait, SegmentData, collect_all_segments
│       ├── model.rs  # ModelSegment
│       ├── directory.rs
│       ├── git.rs
│       ├── context_window.rs
│       ├── usage.rs
│       ├── cost.rs
│       ├── session.rs
│       ├── output_style.rs
│       └── update.rs
│
├── ui/               # Presentation layer (ratatui TUI)
│   ├── mod.rs        # Module exports, run_configurator()
│   ├── app.rs        # App struct, main event loop
│   ├── events.rs     # Event handling
│   ├── layout.rs     # Layout definitions
│   ├── main_menu.rs  # Main menu screen
│   ├── components/   # Reusable UI components
│   │   ├── mod.rs
│   │   ├── color_picker.rs
│   │   ├── editor.rs
│   │   ├── help.rs
│   │   ├── icon_selector.rs
│   │   ├── name_input.rs
│   │   ├── preview.rs
│   │   ├── segment_list.rs
│   │   ├── separator_editor.rs
│   │   ├── settings.rs
│   │   └── theme_selector.rs
│   └── themes/       # Theme presets
│       ├── mod.rs
│       ├── presets.rs
│       └── theme_*.rs
│
└── utils/            # Utility functions
    ├── mod.rs
    ├── credentials.rs
    └── claude_code_patcher.rs
```

---

## Module Organization

### Config Layer (`src/config/`)

- **types.rs**: All configuration structs with serde derives
  - `Config`: Main configuration container
  - `SegmentConfig`: Per-segment settings
  - `StyleConfig`: Global style settings
  - Input/Output data structures (`InputData`, `Model`, etc.)

- **loader.rs**: File I/O operations
  - `Config::load()`, `Config::save()`
  - Theme directory management

### Core Layer (`src/core/`)

- **segments/**: Each segment is self-contained
  - Implements `Segment` trait with `collect()` and `id()` methods
  - Returns `SegmentData { primary, secondary, metadata }`

- **statusline.rs**: Rendering logic
  - `StatusLineGenerator`: Handles ANSI color codes, separators
  - `collect_all_segments()`: Orchestrates segment collection

### UI Layer (`src/ui/`)

- **app.rs**: Main application state and event loop
- **components/**: Each component is a self-contained widget
  - Has its own state struct (e.g., `ColorPickerComponent`)
  - Renders via `.render(f, area)` method

---

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Modules | snake_case | `statusline.rs`, `color_picker.rs` |
| Structs | PascalCase | `StatusLineGenerator`, `SegmentData` |
| Enums | PascalCase | `StyleMode`, `SegmentId`, `GitStatus` |
| Functions | snake_case | `collect_all_segments()`, `get_git_info()` |
| Constants | SCREAMING_SNAKE_CASE | (none in current codebase) |
| Methods | snake_case | `.render_segment()`, `.apply_color()` |

### File Naming

- Module files: `mod.rs` for directory modules
- Feature files: descriptive snake_case (e.g., `context_window.rs`)
- Theme files: `theme_<name>.rs` (e.g., `theme_gruvbox.rs`)

---

## Adding New Features

### Adding a New Segment

1. **Create segment file**: `src/core/segments/<name>.rs`
   - Define struct, implement `Default` and `Segment` trait
   - Return `Option<SegmentData>` from `collect()` - use `None` for graceful degradation

2. **Register in core layer**:
   - Add to `mod.rs` exports
   - Add branch in `collect_all_segments()` in `statusline.rs`

3. **Add configuration**:
   - Add `SegmentId` variant in `src/config/types.rs`

4. **Update UI components** (required for TUI):
   - `src/ui/app.rs` - Add match arm for new `SegmentId`
   - `src/ui/components/preview.rs` - Add mock data for preview
   - `src/ui/components/segment_list.rs` - Add segment name display
   - `src/ui/components/settings.rs` - Add segment name display

5. **Add to all theme presets**:
   - Each `src/ui/themes/theme_*.rs` needs a `<name>_segment()` function
   - Segment is typically disabled by default in themes

**Pattern: Graceful Degradation**
```rust
// In collect() - return None when dependencies not available
fn collect(&self, input: &InputData) -> Option<SegmentData> {
    let info = self.get_info(&input.workspace.current_dir)?;  // Returns None if not found
    // ... build SegmentData
    Some(SegmentData { primary, secondary, metadata })
}
```

### Adding a New UI Component

1. Create `src/ui/components/<name>.rs`
2. Define component struct with state
3. Implement `.render()` method
4. Add to `mod.rs` exports
5. Integrate in `app.rs` if needed

### Adding a New Theme

1. Create `src/ui/themes/theme_<name>.rs`
2. Implement theme configuration
3. Register in `presets.rs` theme list
