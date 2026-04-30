use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

/// Trellis session file structure
#[derive(Debug, Deserialize)]
struct SessionFile {
    current_task: Option<String>,
}

/// Task JSON structure
#[derive(Debug, Deserialize)]
struct TaskFile {
    title: Option<String>,
    name: Option<String>,
    status: Option<String>,
    priority: Option<String>,
}

pub struct TrellisSegment;

impl Default for TrellisSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl TrellisSegment {
    pub fn new() -> Self {
        Self
    }

    /// Find .trellis directory by walking up from the given directory
    fn find_trellis_dir(start_dir: &Path) -> Option<std::path::PathBuf> {
        let mut current = start_dir;

        loop {
            let trellis_dir = current.join(".trellis");
            if trellis_dir.is_dir() {
                return Some(trellis_dir);
            }

            current = current.parent()?;
        }
    }

    /// Get context key from environment variables
    /// Priority: TRELLIS_CONTEXT_ID > CLAUDE_SESSION_ID
    fn get_context_key() -> Option<String> {
        // Check for explicit Trellis context ID first
        if let Ok(context_id) = env::var("TRELLIS_CONTEXT_ID") {
            if !context_id.is_empty() {
                return Some(context_id);
            }
        }

        // Check for Claude session ID
        if let Ok(session_id) = env::var("CLAUDE_SESSION_ID") {
            if !session_id.is_empty() {
                return Some(format!("claude_{}", session_id));
            }
        }

        None
    }

    /// Read current task from session file or .current-task file
    fn get_current_task_ref(trellis_dir: &Path) -> Option<String> {
        // Try session file first
        if let Some(context_key) = Self::get_context_key() {
            let session_file = trellis_dir
                .join(".runtime")
                .join("sessions")
                .join(format!("{}.json", context_key));

            if session_file.exists() {
                if let Ok(content) = fs::read_to_string(&session_file) {
                    if let Ok(session) = serde_json::from_str::<SessionFile>(&content) {
                        if let Some(task_ref) = session.current_task {
                            return Some(task_ref);
                        }
                    }
                }
            }
        }

        // Fallback to .current-task file
        let current_task_file = trellis_dir.join(".current-task");
        if current_task_file.exists() {
            if let Ok(content) = fs::read_to_string(&current_task_file) {
                let task_ref = content.trim().to_string();
                if !task_ref.is_empty() {
                    return Some(task_ref);
                }
            }
        }

        None
    }

    /// Resolve task reference to task directory path
    fn resolve_task_dir(trellis_dir: &Path, task_ref: &str) -> std::path::PathBuf {
        let normalized = task_ref.replace('\\', "/");
        let normalized = normalized.trim_start_matches("./");

        // If it's an absolute path, use it directly
        let path = Path::new(normalized);
        if path.is_absolute() {
            return path.to_path_buf();
        }

        // If it starts with .trellis/, use it relative to parent
        if normalized.starts_with(".trellis/") {
            return trellis_dir.parent().unwrap_or(trellis_dir).join(normalized);
        }

        // Otherwise, treat it as a task name under tasks/
        trellis_dir.join("tasks").join(normalized)
    }

    /// Load task data from task.json
    fn load_task(task_dir: &Path) -> Option<TaskFile> {
        let task_file = task_dir.join("task.json");
        if !task_file.exists() {
            return None;
        }

        let content = fs::read_to_string(&task_file).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Count active (non-archived) tasks
    fn count_active_tasks(trellis_dir: &Path) -> usize {
        let tasks_dir = trellis_dir.join("tasks");
        if !tasks_dir.is_dir() {
            return 0;
        }

        let mut count = 0;
        if let Ok(entries) = fs::read_dir(&tasks_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Skip archive directories
                if path.file_name().is_some_and(|n| n == "archive") {
                    continue;
                }
                // Count directories with valid task.json
                if path.is_dir() && path.join("task.json").exists() {
                    count += 1;
                }
            }
        }

        count
    }

    /// Read developer name from .developer file
    fn get_developer(trellis_dir: &Path) -> Option<String> {
        let dev_file = trellis_dir.join(".developer");
        if !dev_file.exists() {
            return None;
        }

        let content = fs::read_to_string(&dev_file).ok()?;
        for line in content.lines() {
            if let Some(stripped) = line.strip_prefix("name=") {
                return Some(stripped.trim().to_string());
            }
        }

        // Fallback to first non-empty line
        content
            .lines()
            .next()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    }
}

impl Segment for TrellisSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        // Find .trellis directory
        let working_dir = Path::new(&input.workspace.current_dir);
        let trellis_dir = Self::find_trellis_dir(working_dir)?;

        // Get developer and task count (always show if Trellis exists)
        let developer = Self::get_developer(&trellis_dir);
        let task_count = Self::count_active_tasks(&trellis_dir);

        // Try to get current task
        let task_ref = Self::get_current_task_ref(&trellis_dir);
        let task_info = task_ref.as_ref().and_then(|tr| {
            let task_dir = Self::resolve_task_dir(&trellis_dir, tr);
            let task = Self::load_task(&task_dir)?;
            Some((task, tr.clone()))
        });

        // Build output
        let mut metadata = HashMap::new();

        if let Some((ref task, _)) = task_info {
            metadata.insert("has_task".to_string(), "true".to_string());
            metadata.insert(
                "priority".to_string(),
                task.priority.clone().unwrap_or_else(|| "P2".to_string()),
            );
            metadata.insert(
                "status".to_string(),
                task.status.clone().unwrap_or_else(|| "unknown".to_string()),
            );
        }

        if let Some(ref dev) = developer {
            metadata.insert("developer".to_string(), dev.clone());
        }

        metadata.insert("task_count".to_string(), task_count.to_string());

        // Build primary text
        let primary = if let Some((ref task, _)) = task_info {
            let priority = task.priority.as_deref().unwrap_or("P2");
            let title = task
                .title
                .as_ref()
                .or(task.name.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("unknown");
            let status = task.status.as_deref().unwrap_or("unknown");

            format!("[{}] {} ({})", priority, title, status)
        } else {
            // No active task
            String::new()
        };

        // Build secondary text with developer and task count
        let mut secondary_parts = Vec::new();

        if let Some(ref dev) = developer {
            secondary_parts.push(dev.clone());
        }

        if task_count > 0 {
            secondary_parts.push(format!(
                "{} task{}",
                task_count,
                if task_count > 1 { "s" } else { "" }
            ));
        }

        let secondary = secondary_parts.join(" · ");

        // Only return data if we have something to show
        if primary.is_empty() && secondary.is_empty() {
            return None;
        }

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Trellis
    }
}
