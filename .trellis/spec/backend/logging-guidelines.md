# Logging Guidelines

> How logging is done in this project.

---

## Overview

This project does not use a formal logging framework. Instead, it uses:

1. **`println!`** for normal output (statusline, user-facing messages)
2. **`eprintln!`** for warnings and errors
3. **Structured output** for CLI operation feedback

The statusline output is the primary product - it goes to stdout and is consumed by the shell prompt. All other messages go to stderr to avoid interfering with the statusline.

---

## Output Destinations

### stdout (Standard Output)

Reserved for the statusline output only. This is consumed by the shell prompt.

```rust
// src/main.rs:74
println!("{}", statusline);
```

### stderr (Standard Error)

Used for warnings, errors, and diagnostic messages:

```rust
// src/ui/app.rs:70-72
if let Err(e) = crate::config::loader::ConfigLoader::init_themes() {
    eprintln!("Warning: Failed to initialize themes: {}", e);
}
```

### Progress Messages

For CLI operations with user feedback:

```rust
// src/main.rs:19-36
println!("🔧 Claude Code Context Warning Disabler");
println!("Target file: {}", claude_path);
println!("📦 Created backup: {}", backup_path);
println!("\n🔄 Applying patches...");
```

---

## Output Patterns

### Pattern 1: Statusline Output

The primary output - clean, parseable statusline string:

```rust
// src/main.rs:71-74
let generator = StatusLineGenerator::new(config);
let statusline = generator.generate(segments_data);
println!("{}", statusline);
```

### Pattern 2: Operation Progress

For multi-step operations, show progress:

```rust
// src/main.rs:19-36
println!("🔧 Claude Code Context Warning Disabler");
println!("Target file: {}", claude_path);
// ... operation ...
println!("📦 Created backup: {}", backup_path);
println!("\n🔄 Applying patches...");
// ... patches ...
println!("💡 To restore warnings, replace your cli.js with the backup file:");
println!("   cp {} {}", backup_path, claude_path);
```

### Pattern 3: Warning Messages

Use `eprintln!` for non-fatal issues:

```rust
// src/ui/app.rs:70-72
if let Err(e) = crate::config::loader::ConfigLoader::init_themes() {
    eprintln!("Warning: Failed to initialize themes: {}", e);
}
```

### Pattern 4: Summary Output

For batch operations, show summary:

```rust
// src/utils/claude_code_patcher.rs:822-843
pub fn print_summary(results: &[(&str, bool)]) {
    println!("\n📊 Patch Results:");
    for (name, success) in results {
        if *success {
            println!("  ✅ {}", name);
        } else {
            println!("  ❌ {}", name);
        }
    }

    let success_count = results.iter().filter(|(_, s)| *s).count();
    let total_count = results.len();

    if success_count == total_count {
        println!("\n✅ All {} patches applied successfully!", total_count);
    } else {
        println!(
            "\n⚠️ {}/{} patches applied successfully",
            success_count, total_count
        );
    }
}
```

### Pattern 5: Debug Output

For debugging during development (use sparingly):

```rust
// src/utils/claude_code_patcher.rs:145-149
println!(
    "Found Spinner component with spinnerTip and overrideMessage at {}-{}",
    node.start_byte(),
    node.end_byte()
);
```

### Pattern 6: Theme Creation Feedback

When creating files, inform the user:

```rust
// src/config/loader.rs:54
println!("Created theme file: {}", theme_path.display());
```

---

## Unicode Symbols in Output

The project uses Unicode symbols for visual clarity:

| Symbol | Usage |
|--------|-------|
| `🔧` | Tool/configuration |
| `📦` | Package/archive |
| `🔄` | Processing/in-progress |
| `💡` | Tip/suggestion |
| `📊` | Statistics/summary |
| `✅` | Success |
| `❌` | Failure |
| `⚠️` | Warning |

```rust
// Example usage
println!("✅ All {} patches applied successfully!", total_count);
println!("⚠️ Could not disable context low warnings");
```

---

## What to Output

### Always Output

1. **Statusline** - The primary product output
2. **Operation results** - Success/failure of CLI operations
3. **User-requested information** - Config print, version, etc.

### Output for Interactive Mode

1. **Progress messages** - What's happening
2. **Summary** - Results of batch operations
3. **Tips** - Helpful suggestions for next steps

### Output for Error Conditions

1. **What failed** - Clear description of the problem
2. **Context** - Relevant paths, values
3. **Suggestion** - How to fix (if applicable)

```rust
// Good error output
eprintln!("Warning: Failed to initialize themes: {}", e);
println!("💡 To restore warnings, replace your cli.js with the backup file:");
```

---

## What NOT to Output

### Never Output

1. **Secrets/tokens** - OAuth tokens, API keys
2. **Internal state dumps** - Raw structs (use Debug trait instead)
3. **Verbose debug info** - In production builds

### Conditional Output

Debug output should be conditional or removed before release:

```rust
// During development
println!("DEBUG: Parsed {} segments", segments.len());

// Before release - remove or make conditional
#[cfg(debug_assertions)]
println!("DEBUG: Parsed {} segments", segments.len());
```

---

## Future: Adding a Logging Framework

If a logging framework is needed, consider:

### Recommended: `tracing` crate

```rust
use tracing::{info, warn, error, debug};

// Add to Cargo.toml
// [dependencies]
// tracing = "0.1"
// tracing-subscriber = "0.3"

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting application");
    warn!("Non-critical issue: {}", issue);
    error!("Failed to load config: {}", err);
}
```

### Log Levels

| Level | When to Use |
|-------|-------------|
| `error!` | Unrecoverable errors |
| `warn!` | Non-critical issues |
| `info!` | Important operations |
| `debug!` | Detailed progress |
| `trace!` | Very verbose debugging |

### Integration with stdout/stderr

```rust
// Statusline still goes to stdout
println!("{}", statusline);

// Logs go to stderr via tracing
info!("Processing {} segments", segments.len());
```

---

## Common Mistakes

### Mixing stdout and stderr

```rust
// BAD - Debug output on stdout interferes with statusline
println!("DEBUG: Processing...");  // Goes to statusline!
println!("{}", statusline);

// GOOD - Debug output on stderr
eprintln!("DEBUG: Processing...");
println!("{}", statusline);
```

### Excessive Output

```rust
// BAD - Too verbose for normal operation
for segment in &segments {
    println!("Processing segment: {:?}", segment);
}

// GOOD - Summarize
println!("Processed {} segments", segments.len());
```

### Missing Context in Errors

```rust
// BAD - No context
eprintln!("Error: {}", e);

// GOOD - Include what operation failed
eprintln!("Failed to load config from {}: {}", path.display(), e);
```
