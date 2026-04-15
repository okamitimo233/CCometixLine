# Error Handling

> How errors are handled in this project.

---

## Overview

This project uses Rust's standard error handling patterns:

1. **`Result<T, Box<dyn std::error::Error>>`** for fallible operations with various error types
2. **`Option<T>`** for operations that may return nothing
3. **`unwrap_or_else()`** for providing default values on error
4. **`?` operator** for error propagation

Key principle: The CLI tool should be resilient and never crash. Errors are gracefully handled with fallbacks.

---

## Error Types

### Dynamic Error Trait

The project uses `Box<dyn std::error::Error>` for flexible error handling across different error sources:

```rust
// src/main.rs:7
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...
}
```

```rust
// src/config/loader.rs:21-25
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

This allows combining errors from:
- `std::io::Error` (file operations)
- `toml::de::Error` (TOML parsing)
- `serde_json::Error` (JSON parsing)
- Custom error strings

### Custom Result Types

The `InitResult` enum provides specific success states:

```rust
// src/config/loader.rs:6-12
#[derive(Debug)]
pub enum InitResult {
    /// Config was created at the given path
    Created(PathBuf),
    /// Config already existed at the given path
    AlreadyExists(PathBuf),
}
```

---

## Error Handling Patterns

### Pattern 1: Fallback to Defaults

Use `unwrap_or_else()` when errors should result in default values:

```rust
// src/main.rs:42
let mut config = Config::load().unwrap_or_else(|_| Config::default());
```

```rust
// src/config/loader.rs:17-19
pub fn load() -> Config {
    Config::load().unwrap_or_else(|_| Config::default())
}
```

This ensures the application continues running even if config loading fails.

### Pattern 2: Error Propagation with `?`

Use `?` operator to propagate errors up the call stack:

```rust
// src/config/loader.rs:132-143
pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Self::get_config_path();

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(self)?;
    fs::write(config_path, content)?;
    Ok(())
}
```

### Pattern 3: Silent Error Handling

For non-critical operations, use `let _ =` to explicitly discard errors:

```rust
// src/config/loader.rs:78
let _ = Self::init_themes_silent();
```

```rust
// src/ui/app.rs:70-72
if let Err(e) = crate::config::loader::ConfigLoader::init_themes() {
    eprintln!("Warning: Failed to initialize themes: {}", e);
}
```

### Pattern 4: Command Output Handling

For external command execution (like git), handle failures gracefully:

```rust
// src/core/segments/git.rs:67-74
fn is_git_repository(&self, working_dir: &str) -> bool {
    Command::new("git")
        .args(["--no-optional-locks", "rev-parse", "--git-dir"])
        .current_dir(working_dir)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
```

```rust
// src/core/segments/git.rs:139-151
fn get_commit_count(&self, working_dir: &str, range: &str) -> u32 {
    let output = Command::new("git")
        .args(["--no-optional-locks", "rev-list", "--count", range])
        .current_dir(working_dir)
        .output();

    match output {
        Ok(output) if output.status.success() => String::from_utf8(output.stdout)
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0),
        _ => 0,
    }
}
```

### Pattern 5: Option Chaining for Optional Data

Use `Option` combinators for handling missing data:

```rust
// src/core/segments/git.rs:154-171
fn get_sha(&self, working_dir: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["--no-optional-locks", "rev-parse", "--short=7", "HEAD"])
        .current_dir(working_dir)
        .output()
        .ok()?;

    if output.status.success() {
        let sha = String::from_utf8(output.stdout).ok()?.trim().to_string();
        if sha.is_empty() {
            None
        } else {
            Some(sha)
        }
    } else {
        None
    }
}
```

---

## Validation Errors

The `check()` method validates configuration and returns meaningful error messages:

```rust
// src/config/loader.rs:177-192
pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Basic validation
    if self.segments.is_empty() {
        return Err("No segments configured".into());
    }

    // Validate segment IDs are unique
    let mut seen_ids = std::collections::HashSet::new();
    for segment in &self.segments {
        if !seen_ids.insert(segment.id) {
            return Err(format!("Duplicate segment ID: {:?}", segment.id).into());
        }
    }

    Ok(())
}
```

---

## Common Mistakes

### Using `.unwrap()` in Production Code

```rust
// BAD - Will panic on error
let config = Config::load().unwrap();

// GOOD - Gracefully handle error with default
let config = Config::load().unwrap_or_else(|_| Config::default());
```

### Ignoring External Command Errors

```rust
// BAD - Assumes git always succeeds
let output = Command::new("git").args(...).output().unwrap();

// GOOD - Handle failure gracefully
let output = Command::new("git").args(...).output();
match output {
    Ok(o) if o.status.success() => { /* process */ },
    _ => { /* fallback behavior */ },
}
```

### Not Validating JSON Input

```rust
// BAD - Assumes input is always valid
let input: InputData = serde_json::from_reader(stdin.lock()).unwrap();

// GOOD - Return error for invalid input
let input: InputData = serde_json::from_reader(stdin.lock())?;
```

### Using `expect()` Without Context

```rust
// BAD - Generic panic message
let home = dirs::home_dir().expect("home dir");

// GOOD - Provide context or use fallback
let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
```

---

## Error Messages for Users

When displaying errors to users, use `eprintln!` for warnings and provide actionable information:

```rust
// src/main.rs:19-21
println!("🔧 Claude Code Context Warning Disabler");
println!("Target file: {}", claude_path);

// src/ui/app.rs:197-200
if let Err(e) = app.save_config() {
    app.status_message = Some(format!("Failed to save config: {}", e));
}
```

For CLI patcher operations, show detailed progress:

```rust
// src/utils/claude_code_patcher.rs:633-643
if success_count == total_count {
    println!("\n✅ All {} patches applied successfully!", total_count);
} else {
    println!(
        "\n⚠️ {}/{} patches applied successfully",
        success_count, total_count
    );
}
```
