# Database Guidelines

> ORM, migrations, query patterns, naming conventions.

---

## Overview

CCometixLine does **not use a database**. This is a stateless CLI/TUI application that:

1. Reads configuration from TOML files
2. Reads input from stdin (JSON)
3. Writes configuration to TOML files
4. Writes temporary state to JSON files (update checks)

---

## Data Persistence Patterns

### Configuration Storage

**Location**: `~/.claude/ccline/config.toml`

```rust
// src/config/loader.rs:115-129
impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        if !config_path.exists() {
            return Ok(Config::default());
        }
        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
}
```

### State Storage

**Location**: `~/.claude/ccline/.update_state.json`

```rust
// src/updater.rs:125-136
pub fn save(&self) -> Result<(), std::io::Error> {
    let config_dir = dirs::home_dir()
        .unwrap_or_default()
        .join(".claude")
        .join("ccline");

    std::fs::create_dir_all(&config_dir)?;
    let state_file = config_dir.join(".update_state.json");

    let content = serde_json::to_string_pretty(self)?;
    std::fs::write(&state_file, content)?;
    Ok(())
}
```

### Theme Files

**Location**: `~/.claude/ccline/themes/<theme>.toml`

Themes are stored as individual TOML files with the same structure as config.

---

## Data Structures

### Serde for Serialization

All persistent data structures use serde:

```rust
// src/config/types.rs:5-10
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub style: StyleConfig,
    pub segments: Vec<SegmentConfig>,
    pub theme: String,
}
```

### Handling External Data (stdin)

```rust
// src/main.rs:64-65
let stdin = io::stdin();
let input: InputData = serde_json::from_reader(stdin.lock())?;
```

Input data structures use `#[serde(default)]` for optional fields:

```rust
// src/config/types.rs:123-130
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PromptTokensDetails {
    #[serde(default)]
    pub cached_tokens: Option<u32>,
    #[serde(default)]
    pub audio_tokens: Option<u32>,
}
```

---

## File I/O Conventions

| Operation | Function | Error Handling |
|-----------|----------|----------------|
| Read config | `Config::load()` | Return default on failure |
| Write config | `Config::save()` | Propagate error |
| Read state | `UpdateState::load()` | Return default on failure |
| Write state | `UpdateState::save()` | Ignore errors (`let _ =`) |
| Ensure directory | `fs::create_dir_all()` | Propagate error |

### Path Construction

Always use `PathBuf` and cross-platform path handling:

```rust
// src/config/loader.rs:145-151
fn get_config_path() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".claude").join("ccline").join("config.toml")
    } else {
        PathBuf::from(".claude/ccline/config.toml")
    }
}
```

---

## Migration Pattern (If Needed in Future)

If database-like migrations become necessary (e.g., config format changes):

1. **Version the data structure**:
   ```rust
   #[derive(Deserialize)]
   struct ConfigV1 { ... }
   ```

2. **Detect version on load**:
   ```rust
   fn load() -> Result<Config, Error> {
       let raw: Value = toml::from_str(&content)?;
       let version = raw.get("version").and_then(|v| v.as_u64()).unwrap_or(1);
       match version {
           1 => migrate_from_v1(raw),
           _ => Ok(toml::from_str(&content)?),
       }
   }
   ```

3. **Migrate to current version**:
   ```rust
   fn migrate_from_v1(raw: Value) -> Result<Config, Error> {
       let v1: ConfigV1 = serde_json::from_value(raw)?;
       Ok(Config::from(v1))
   }
   ```

---

## No-Database Design Principle

This project intentionally avoids database dependencies:

- **Rationale**: CLI tools should be stateless and portable
- **Config location**: User's home directory (~/.claude/ccline/)
- **Format**: Human-readable TOML for easy manual editing
- **State**: Minimal JSON state files for specific features
