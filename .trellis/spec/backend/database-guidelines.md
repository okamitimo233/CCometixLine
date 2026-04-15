# Database Guidelines

> Database patterns and conventions for this project.

---

## Overview

**This project does not use a database.**

CCometixLine is a command-line status line generator that operates on a request-response model:

1. Receives JSON input from stdin (Claude Code context)
2. Processes data in memory
3. Outputs formatted status line to stdout

No persistent data storage is required for the core functionality.

---

## Data Persistence Used

### File-Based Configuration

Configuration is stored in TOML files:

- **Location**: `~/.claude/ccline/config.toml`
- **Format**: TOML (human-readable configuration)
- **Loading**: `Config::load()` in `src/config/loader.rs`

```rust
// src/config/loader.rs:116-128
pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Self::get_config_path();

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

### Theme Files

Themes are stored as individual TOML files:

- **Location**: `~/.claude/ccline/themes/*.toml`
- **Pattern**: One file per theme (e.g., `gruvbox.toml`, `nord.toml`)

```rust
// src/config/loader.rs:51-56
let theme_config = crate::ui::themes::ThemePresets::get_theme(theme_name);
let content = toml::to_string_pretty(&theme_config)?;
fs::write(&theme_path, content)?;
```

### Cache Files

API usage data is cached to reduce API calls:

- **Location**: `~/.claude/ccline/.api_usage_cache.json`
- **Format**: JSON with timestamp for validity checking
- **Pattern**: Read-through cache with TTL

```rust
// src/core/segments/usage.rs:77-86
fn load_cache(&self) -> Option<ApiUsageCache> {
    let cache_path = Self::get_cache_path()?;
    if !cache_path.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&cache_path).ok()?;
    serde_json::from_str(&content).ok()
}
```

### State Files

Update check state is persisted:

- **Location**: `~/.claude/ccline/.update_state.json`
- **Purpose**: Track last update check time and available updates

---

## If Database Were Needed

If database functionality were required in the future, the following patterns should be considered:

### Recommended Approach

1. **SQLite** for local storage:
   - Zero-config, embedded database
   - Suitable for CLI tools
   - Use `rusqlite` crate

2. **Async patterns** with `sqlx` if async I/O is needed

### File Structure

```
src/
├── db/
│   ├── mod.rs           # Database module entry
│   ├── connection.rs    # Connection pool management
│   ├── migrations/      # SQL migration files
│   └── repositories/    # Data access layer
```

### Naming Conventions

- **Tables**: `snake_case`, plural (e.g., `sessions`, `configurations`)
- **Columns**: `snake_case` (e.g., `created_at`, `user_id`)
- **Indexes**: `idx_<table>_<columns>` (e.g., `idx_sessions_user_id`)

---

## Common Mistakes

### File Path Handling

Always use `dirs::home_dir()` for user directories, with fallback:

```rust
// GOOD
fn get_config_path() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".claude").join("ccline").join("config.toml")
    } else {
        PathBuf::from(".claude/ccline/config.toml") // fallback
    }
}

// BAD - Hardcoded paths
fn get_config_path() -> PathBuf {
    PathBuf::from("/home/user/.claude/ccline/config.toml")
}
```

### Directory Creation

Always create parent directories before writing:

```rust
// GOOD - Create parent directories
if let Some(parent) = config_path.parent() {
    fs::create_dir_all(parent)?;
}

// BAD - May fail if directory doesn't exist
fs::write(&config_path, content)?;
```

### Error Handling

Don't silently ignore file operations:

```rust
// GOOD - Handle errors appropriately
if let Err(e) = crate::config::loader::ConfigLoader::init_themes() {
    eprintln!("Warning: Failed to initialize themes: {}", e);
}

// BAD - Silent failure with let _
let _ = fs::write(&path, content); // Errors are silently ignored
```
