use chrono::{Local, TimeZone};
use std::path::Path;

use crate::commands::Repository;

use super::fs_ops::FsOps;

/// Log operations for commit and action logging
pub struct LogOps;

impl LogOps {
    /// Write a commit entry to logs
    pub fn write_commit_log(
        child: String,
        parent: String,
        timestamp: u64,
        message: String,
        repo: &Repository,
    ) -> std::io::Result<()> {
        let content = Self::format_commit_entry(&child, &parent, timestamp, &message);

        // Write to main HEAD log
        FsOps::append_to_file(repo.log_head.as_path(), content.as_bytes())?;

        // Write to branch-specific log
        let branch = repo
            .current_branch
            .strip_prefix(repo.gitdir.as_path())
            .unwrap_or(std::path::Path::new("refs/heads/"));
        let branch_log_path = repo.logs.as_path().join(branch);

        // Ensure parent directories exist before writing
        if let Some(parent_dir) = branch_log_path.parent() {
            FsOps::create_dir_all_safe(parent_dir)?;
        }

        FsOps::append_to_file(branch_log_path.as_path(), content.as_bytes())?;

        Ok(())
    }

    /// Write an action log entry
    pub fn write_action_log(action: &str, repo: &Repository) -> std::io::Result<()> {
        FsOps::append_to_file(repo.log_head.as_path(), action.as_bytes())?;
        Ok(())
    }

    /// Format a commit entry in tab-delimited format
    fn format_commit_entry(child: &str, parent: &str, timestamp: u64, message: &str) -> String {
        format!(
            "{}\t{}\t{}\t{}\tcommit\n",
            child, parent, timestamp, message
        )
    }

    /// Read and parse log entries from a file
    pub fn read_log_entries(log_path: &Path) -> std::io::Result<Vec<LogEntry>> {
        let content = FsOps::read_file(log_path)?;
        let entries = content
            .lines()
            .filter_map(|line| Self::parse_log_line(line))
            .collect();
        Ok(entries)
    }

    /// Parse a single log line
    fn parse_log_line(line: &str) -> Option<LogEntry> {
        if line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            Some(LogEntry {
                snapshot: parts[0].to_string(),
                timestamp: parts[2].parse().unwrap_or(0),
                message: parts[3].to_string(),
            })
        } else {
            None
        }
    }

    /// Format a timestamp as readable date
    pub fn format_timestamp(timestamp: u64) -> String {
        Local
            .timestamp_opt(timestamp as i64, 0)
            .unwrap()
            .format("%b %e %Y")
            .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub snapshot: String,
    pub timestamp: u64,
    pub message: String,
}
