# Quality Guidelines

> Code quality standards for backend development.

---

## Overview

This project is a performance-critical CLI tool written in Rust. Code quality focuses on:

1. **Reliability** - Never crash, always handle errors gracefully
2. **Performance** - Fast startup and execution
3. **Maintainability** - Clear module structure and consistent patterns
4. **Readability** - Self-documenting code with clear naming

---

## Forbidden Patterns

### 1. `.unwrap()` in Production Code Paths

Using `.unwrap()` will cause panics on error conditions.

```rust
// BAD - Will panic if config file is missing or invalid
let config = Config::load().unwrap();

// GOOD - Use unwrap_or_else() with default
let config = Config::load().unwrap_or_else(|_| Config::default());

// ACCEPTABLE - In test code only
#[test]
fn test_something() {
    let result = function_under_test().unwrap();
}
```

### 2. `.expect()` Without Meaningful Messages

```rust
// BAD - Vague message
let home = dirs::home_dir().expect("failed");

// GOOD - Provide actionable context
let home = dirs::home_dir().expect("Could not determine home directory");

// BETTER - Use fallback
let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
```

### 3. Silent Error Swallowing

```rust
// BAD - Error is silently ignored
let _ = fs::write(&path, content);

// GOOD - At minimum, log the error
if let Err(e) = fs::write(&path, content) {
    eprintln!("Warning: Failed to write file: {}", e);
}
```

Note: `let _ =` is acceptable when explicitly ignoring a result that is known to be unimportant.

### 4. Ignoring Command Failures

```rust
// BAD - Assumes external command always succeeds
let output = Command::new("git").args(...).output().unwrap();

// GOOD - Check status and handle failure
let output = Command::new("git").args(...).output();
match output {
    Ok(o) if o.status.success() => { /* process output */ },
    _ => { /* graceful fallback */ },
}
```

### 5. Blocking in Async Context

The project uses synchronous I/O. If async is introduced:

```rust
// BAD - Blocking call in async context
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));
}

// GOOD - Use async sleep
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### 6. Hardcoded Paths

```rust
// BAD - Platform-specific hardcoded path
let config_path = "/home/user/.claude/config.toml";

// GOOD - Use dirs crate for cross-platform support
let config_path = dirs::home_dir()
    .map(|h| h.join(".claude").join("config.toml"))
    .unwrap_or_else(|| PathBuf::from(".claude/config.toml"));
```

---

## Required Patterns

### 1. Use Builder Pattern for Configuration

```rust
// src/core/segments/git.rs:32-40
pub struct GitSegment {
    show_sha: bool,
}

impl GitSegment {
    pub fn new() -> Self {
        Self { show_sha: false }
    }

    pub fn with_sha(mut self, show_sha: bool) -> Self {
        self.show_sha = show_sha;
        self
    }
}
```

### 2. Implement Default Trait

```rust
// src/core/segments/git.rs:26-30
impl Default for GitSegment {
    fn default() -> Self {
        Self::new()
    }
}
```

### 3. Use Serde for Serialization

All configuration types should derive `Serialize` and `Deserialize`:

```rust
// src/config/types.rs:5-10
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub style: StyleConfig,
    pub segments: Vec<SegmentConfig>,
    pub theme: String,
}
```

### 4. Use Module Re-exports

Keep public API clean at module level:

```rust
// src/config/mod.rs
pub mod defaults;
pub mod loader;
pub mod models;
pub mod types;

pub use loader::{ConfigLoader, InitResult};
pub use models::*;
pub use types::*;
```

### 5. Document Public APIs

```rust
// src/config/loader.rs:6-12
/// Result of config initialization
#[derive(Debug)]
pub enum InitResult {
    /// Config was created at the given path
    Created(PathBuf),
    /// Config already existed at the given path
    AlreadyExists(PathBuf),
}
```

---

## Testing Requirements

### Current State

The project does not have unit tests. Tests should be added for:

1. **Configuration parsing** - TOML/JSON deserialization
2. **Segment logic** - Data transformation in each segment
3. **StatusLine generation** - Output formatting
4. **Error handling** - Edge cases and failure modes

### Test Structure (When Added)

```
src/
├── config/
│   └── types.rs
│       └── #[cfg(test)] mod tests { ... }
```

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_usage_total_for_cost() {
        let usage = NormalizedUsage {
            total_tokens: 1000,
            ..Default::default()
        };
        assert_eq!(usage.total_for_cost(), 1000);
    }
}
```

### Test Naming Convention

- Test functions should be named `test_<what>_<condition>`
- Use descriptive names that explain the scenario

```rust
#[test]
fn test_config_load_missing_file_returns_default() { ... }

#[test]
fn test_git_segment_clean_repository() { ... }
```

---

## Code Review Checklist

### Before Submitting

- [ ] No `.unwrap()` in production code paths
- [ ] All errors handled with `?` or `unwrap_or_else()`
- [ ] External command failures handled gracefully
- [ ] File paths use `dirs` crate, not hardcoded
- [ ] New types implement `Debug` trait
- [ ] Configuration types derive `Serialize`/`Deserialize`
- [ ] Public functions have documentation comments
- [ ] No unused variables or imports (`cargo check` passes)
- [ ] Code formatted with `cargo fmt`
- [ ] Clippy warnings addressed (`cargo clippy`)

### For Segment Implementations

- [ ] Implements `Segment` trait
- [ ] Returns `Option<SegmentData>` for graceful None handling
- [ ] Registered in `collect_all_segments()`
- [ ] SegmentId added to enum

### For UI Components

- [ ] State managed in `App` struct
- [ ] Events handled in main event loop
- [ ] Preview updated after changes

---

## Code Style

### Imports

Group imports logically, separated by blank lines:

```rust
// Standard library
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// External crates
use serde::{Deserialize, Serialize};

// Internal modules
use crate::config::{Config, SegmentId};
use super::{Segment, SegmentData};
```

### Function Length

- Keep functions under 50 lines when possible
- Extract helper functions for complex logic
- Use descriptive names for extracted functions

### Comments

- Use `///` for documentation comments on public items
- Use `//` for inline comments explaining non-obvious logic
- Prefer self-documenting code over comments

```rust
// GOOD - Self-documenting code
fn visible_width(text: &str) -> usize { ... }

// GOOD - Comment explaining non-obvious logic
// Parse once and reuse for all patches
fn parse_tree(&self) -> Option<Tree> { ... }
```
