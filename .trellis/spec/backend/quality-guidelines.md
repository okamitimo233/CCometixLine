# Quality Guidelines

> Code quality standards for backend development.

---

## Overview

CCometixLine follows standard Rust best practices with emphasis on:

- Type safety through strong typing
- Explicit error handling
- Clean module boundaries
- No panics in production code paths

---

## Forbidden Patterns

### ❌ Never Use `.unwrap()` in Production Paths

```rust
// Bad: Will panic on failure
let config = Config::load().unwrap();

// Good: Handle gracefully
let config = Config::load().unwrap_or_else(|_| Config::default());
```

**Exception**: Test code may use `.unwrap()` freely.

### ❌ Never Use `.expect()` Without Context

```rust
// Bad: Generic panic message
let data = fs::read_to_string(path).expect("failed");

// Good: Actionable context
let data = fs::read_to_string(path).expect(&format!("Failed to read {}", path.display()));
```

**Better**: Use `?` operator and propagate errors.

### ❌ Never Panic in Library Code

Functions in `src/lib.rs` exports should never panic:

```rust
// Bad: Panics on invalid input
pub fn parse(input: &str) -> Config {
    toml::from_str(input).unwrap()
}

// Good: Returns Result
pub fn parse(input: &str) -> Result<Config, Box<dyn std::error::Error>> {
    Ok(toml::from_str(input)?)
}
```

### ❌ Never Use `unsafe` Without Documentation

If `unsafe` is required, document the safety invariants:

```rust
// SAFETY: We ensure the pointer is valid because...
unsafe { ... }
```

---

## Required Patterns

### ✅ Use `#[derive]` for Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // ...
}
```

Always derive at least `Debug` for debugging.

### ✅ Implement `Default` When Appropriate

```rust
#[derive(Default)]
pub struct ModelSegment;

impl Default for GitSegment {
    fn default() -> Self {
        Self::new()
    }
}
```

### ✅ Use Builder Pattern for Optional Configuration

```rust
// src/core/segments/git.rs:37-40
impl GitSegment {
    pub fn with_sha(mut self, show_sha: bool) -> Self {
        self.show_sha = show_sha;
        self
    }
}
```

### ✅ Use `?` for Error Propagation

```rust
// src/config/loader.rs:22-25
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

### ✅ Use `Option` for Genuinely Optional Values

```rust
pub struct ColorConfig {
    pub icon: Option<AnsiColor>,  // Color may not be set
    pub text: Option<AnsiColor>,
    pub background: Option<AnsiColor>,
}
```

---

## Testing Requirements

### Current State

The project has minimal unit tests. When adding tests:

### Test Structure

```
src/
├── some_module.rs
└── some_module/
    └── tests.rs  (or inline #[cfg(test)] modules)
```

### Inline Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_width_strips_ansi() {
        let text = "\x1b[31mhello\x1b[0m";
        assert_eq!(visible_width(text), 5);
    }
}
```

### Test Coverage Focus

1. **Segment logic** - Test data extraction and formatting
2. **Color handling** - Test ANSI code generation
3. **Config parsing** - Test TOML serialization/deserialization
4. **Error paths** - Test fallback behavior

---

## Code Review Checklist

### Before Submitting

- [ ] `cargo check` passes
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt --check` passes
- [ ] No `.unwrap()` in non-test code paths
- [ ] New public functions have documentation comments
- [ ] Error messages are actionable

### Reviewer Should Check

- [ ] Module boundaries are respected
- [ ] Error handling is explicit, not hidden
- [ ] No unnecessary allocations or clones
- [ ] Config changes include migration path if needed
- [ ] New dependencies are justified

---

## Linting Standards

### Clippy Configuration

The project follows default Clippy recommendations. Key lints to watch:

- `clippy::unwrap_used` - Warn in non-test code
- `clippy::expect_used` - Warn without context
- `clippy::uninlined_format_args` - Use inline format strings

### Running Lints

```bash
cargo clippy -- -D warnings
cargo fmt -- --check
```

---

## Documentation Standards

### Public Items Must Have Docs

```rust
/// Strip ANSI escape sequences and return visible text length
fn visible_width(text: &str) -> usize {
    // ...
}
```

### Internal Items Can Be Undocumented

Private functions and structs don't require documentation, but add a brief comment if logic is non-obvious.

### Avoid Obvious Comments

```rust
// Bad: Explains what (obvious from code)
// Check if enabled
if config.enabled { ... }

// Good: Explains why (non-obvious)
// Skip disabled segments to avoid unnecessary API requests
if !segment_config.enabled {
    continue;
}
```
