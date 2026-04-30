# Logging Guidelines

> How logging is done in this project.

---

## Overview

CCometixLine does **not use a formal logging library**. As a CLI/TUI tool, it uses simple output methods:

- **stdout**: Normal output (statusline, config print)
- **stderr**: Warnings and errors (`eprintln!`)
- **TUI status**: In-app status messages

This lightweight approach is intentional for a CLI tool that pipes output to other processes.

---

## Output Methods

### stdout (`println!`)

Used for primary output that may be captured by other processes:

```rust
// src/main.rs:74 - Final statusline output
println!("{}", statusline);

// src/config/loader.rs:54 - User feedback during init
println!("Created theme file: {}", theme_path.display());
```

### stderr (`eprintln!`)

Used for warnings and errors that should not mix with stdout:

```rust
// src/ui/app.rs:70-71
if let Err(e) = crate::config::loader::ConfigLoader::init_themes() {
    eprintln!("Warning: Failed to initialize themes: {}", e);
}
```

### Status Messages (TUI only)

In the TUI configurator, use `status_message`:

```rust
// src/ui/app.rs:199-201
self.status_message = Some("Configuration saved to config.toml!".to_string());
```

---

## Log Levels (Informal)

Since there's no logging crate, follow these conventions:

| Level | Method | When to Use |
|-------|--------|-------------|
| Info | `println!` | Primary output, user-requested information |
| Warning | `eprintln!("Warning: ...")` | Non-fatal issues, fallback behavior |
| Error | `eprintln!("Error: ...")` | Fatal errors before exit |
| Debug | commented or conditional | Development debugging (remove before merge) |

---

## What to Log

### ✅ Log These

- Successful operations (user-initiated saves, theme switches)
- Configuration file paths (for debugging)
- Version information
- Git command results (for segment debugging)

### Example

```rust
// src/main.rs:19-20 - User-facing operation feedback
println!("🔧 Claude Code Context Warning Disabler");
println!("Target file: {}", claude_path);
```

---

## What NOT to Log

### ❌ Never Log These

- API keys or tokens
- User file contents (except config the user explicitly saves)
- Internal state unless debugging

### Example of Safe Logging

```rust
// Good: Log operation, not content
println!("Created backup: {}", backup_path);

// Bad: Would log potentially sensitive content
// println!("Config contents: {:?}", config);
```

---

## Debugging Output

For temporary debugging during development:

```rust
// Use eprintln for debug output (goes to stderr, won't break piping)
eprintln!("DEBUG: segment data = {:?}", segment_data);

// Remove before committing, or use conditional compilation:
#[cfg(debug_assertions)]
eprintln!("DEBUG: {:?}", data);
```

---

## TUI Status Messages

In the configurator app, always update `status_message` for user feedback:

```rust
// src/ui/app.rs:645
self.status_message = Some(format!("Reset {} theme to defaults", current_theme));
```

### Good Status Messages

- **Action + Result**: "Configuration saved to config.toml!"
- **State Change**: "Model segment enabled"
- **Error Context**: "Failed to save theme: permission denied"

### Bad Status Messages

- **Too vague**: "Done"
- **No context**: "Error"
- **Internal jargon**: "SegmentConfig::save() returned Err"

---

## Future Considerations

If logging needs grow, consider:

1. **log crate**: Standard facade for Rust logging
2. **env_logger**: Runtime log level control via `RUST_LOG`
3. **Structured logging**: For machine-readable output

Current recommendation: Keep it simple until needs arise.
