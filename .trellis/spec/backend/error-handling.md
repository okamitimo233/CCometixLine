# Error Handling

> How errors are handled in this project.

---

## Overview

CCometixLine uses Rust's standard `Result` type with `Box<dyn std::error::Error>` as the general error container. The codebase favors graceful degradation over panics.

---

## Error Types

### Primary Error Signature

```rust
Result<T, Box<dyn std::error::Error>>
```

Used throughout the codebase for functions that can fail:
- File I/O operations
- Configuration loading/saving
- Network requests (update checks)
- Terminal operations

### Specific Error Patterns

| Location | Error Handling Approach |
|----------|------------------------|
| `main.rs` | Returns `Result<(), Box<dyn std::error::Error>>` from main |
| `Config::load()` | Returns `Result<Config, Box<dyn std::error::Error>>` |
| Segment `collect()` | Returns `Option<SegmentData>`, uses `?` internally |

---

## Error Handling Patterns

### 1. Graceful Fallback (Default Values)

```rust
// src/main.rs:42
let mut config = Config::load().unwrap_or_else(|_| Config::default());
```

When config loading fails, use defaults instead of crashing.

### 2. Option Chaining for Optional Operations

```rust
// src/core/segments/git.rs:76-103
fn get_branch(&self, working_dir: &str) -> Option<String> {
    if let Ok(output) = Command::new("git")... {
        if output.status.success() {
            let branch = String::from_utf8(output.stdout).ok()?.trim().to_string();
            if !branch.is_empty() {
                return Some(branch);
            }
        }
    }
    // Fallback to another method...
    None
}
```

Use `Option` chaining when an operation is genuinely optional.

### 3. Silent Failure for Non-Critical Operations

```rust
// src/updater.rs:70-71
let _ = state.save();
```

Using `let _ =` to explicitly ignore errors for non-critical operations.

### 4. Warning Messages for Initialization Failures

```rust
// src/ui/app.rs:70-71
if let Err(e) = crate::config::loader::ConfigLoader::init_themes() {
    eprintln!("Warning: Failed to initialize themes: {}", e);
}
```

Print warnings but continue execution for initialization failures.

### 5. Early Return with `?` Operator

```rust
// src/config/loader.rs:22-25
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

Propagate errors upward using `?` for functions that should fail as a unit.

---

## API Error Responses

Not applicable - this is a CLI/TUI application. External interactions:

1. **stdin parsing**: Uses `serde_json::from_reader()` - errors propagate to main
2. **git commands**: Failures result in `Option::None`, not errors
3. **update checks**: Network failures result in `UpdateStatus::Idle`

---

## Common Mistakes

### ❌ Don't: Panic on missing resources

```rust
// Bad
let config = Config::load().expect("Config must exist");
```

### ✅ Do: Provide fallbacks

```rust
// Good
let config = Config::load().unwrap_or_else(|_| Config::default());
```

### ❌ Don't: Ignore errors silently without thought

```rust
// Bad
config.save(); // Returns Result, ignored
```

### ✅ Do: Explicit silent handling when appropriate

```rust
// Good
let _ = config.save(); // Explicitly acknowledge we're ignoring the result
```

### ❌ Don't: Use `.unwrap()` on fallible operations in production code

```rust
// Bad
let data = serde_json::from_str(&content).unwrap();
```

### ✅ Do: Propagate or handle the error

```rust
// Good
let data = serde_json::from_str(&content)?;
// Or provide a default
let data = serde_json::from_str(&content).unwrap_or_default();
```

---

## Error Context Guidelines

When adding new error-throwing code:

1. **User-facing errors**: Include actionable context
   ```rust
   Err(format!("Failed to save config to {}: {}", path.display(), e).into())
   ```

2. **Internal errors**: Keep messages concise
   ```rust
   Err("Missing version field".into())
   ```

3. **Optional operations**: Prefer `Option` over `Result`
   ```rust
   fn get_git_info(&self) -> Option<GitInfo>  // Not Result<Option<...>>
   ```

---

## Testing Error Paths

When testing, verify error handling:

```rust
#[test]
fn test_missing_config_uses_defaults() {
    let config = Config::load_from_path("/nonexistent/path")
        .unwrap_or_default();
    assert!(!config.segments.is_empty());
}
```
