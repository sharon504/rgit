use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use crate::{commands::Repository, errors::RgitError};

use super::fs_ops::FsOps;

/// Reference and branch operations
pub struct RefOps;

pub enum CheckoutTarget {
    Branch(String),
    Tag(String),
}

impl RefOps {
    /// Figures out if the name is a branch, a tag, or an exact path
    pub fn resolve_checkout_target(
        repo: &Repository,
        name: &str,
    ) -> Result<CheckoutTarget, RgitError> {
        if let Some(branch) = name.strip_prefix("refs/heads/") {
            return Ok(CheckoutTarget::Branch(branch.to_string()));
        }
        if let Some(tag) = name.strip_prefix("refs/tags/") {
            return Ok(CheckoutTarget::Tag(tag.to_string()));
        }
        let branch_exists = FsOps::path_exists(&repo.branches.join(name));
        let tag_exists = FsOps::path_exists(&repo.tags.join(name));

        match (branch_exists, tag_exists) {
            (true, true) => Err(RgitError::AmbiguousReference(name.to_string())),
            (true, false) => Ok(CheckoutTarget::Branch(name.to_string())),
            (false, true) => Ok(CheckoutTarget::Tag(name.to_string())),
            (false, false) => Err(RgitError::RefNotFound(name.to_string())),
        }
    }

    /// Get the latest commit hash in branch
    pub fn get_current_commit_hash(repo: &Repository) -> Result<String, RgitError> {
        if !repo.current_branch.exists() {
            FsOps::create_file_with_content(
                repo.current_branch.as_path(),
                "0".repeat(40).as_bytes(),
            )?;
        }
        let mut current_hash = FsOps::read_file(repo.current_branch.as_path())?;
        if current_hash.is_empty() {
            current_hash = "0".repeat(40);
        }
        Ok(current_hash)
    }

    /// Read the current branch from HEAD file
    pub fn read_current_branch(repo: &mut Repository) -> Result<(), RgitError> {
        let current_branch = FsOps::read_file(repo.head_file.as_path())?;
        repo.current_branch = PathBuf::from(current_branch);
        Ok(())
    }

    /// Write HEAD reference to a branch
    pub fn write_head_reference(repo: &Repository, branch_path: &Path) -> std::io::Result<()> {
        FsOps::create_file_with_content(
            repo.head_file.as_path(),
            branch_path.as_os_str().as_bytes(),
        )
    }

    /// Create a new branch reference
    pub fn create_branch_ref(
        repo: &Repository,
        branch_name: &str,
        commit: &str,
    ) -> Result<(), RgitError> {
        let branch_path = repo.branches.join(branch_name);

        if FsOps::path_exists(&branch_path) {
            return Err(RgitError::BranchAlreadyExists(branch_name.to_string()));
        }

        // Ensure refs/heads directory exists
        FsOps::create_dir_all_safe(repo.branches.as_path())?;
        FsOps::create_file_with_content(branch_path.as_path(), commit.as_bytes())?;

        // Create branch-specific log with parent directory
        let branch_log_path = repo
            .logs
            .as_path()
            .join(format!("refs/heads/{}", branch_name));

        // Ensure logs/refs/heads directory exists (including logs/ parent)
        if let Some(parent_dir) = branch_log_path.parent() {
            FsOps::create_dir_all_safe(parent_dir)?;
        }

        FsOps::create_file_with_content(branch_log_path.as_path(), b"")?;

        Ok(())
    }

    /// Get the commit that a branch points to
    pub fn get_branch_commit(branch_path: &Path) -> Result<String, RgitError> {
        if !FsOps::path_exists(branch_path) {
            return Err(RgitError::BranchNotFound(
                branch_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default(),
            ));
        }

        let commit = FsOps::read_file(branch_path)?;
        Ok(commit.trim().to_string())
    }

    /// Update a branch reference to point to a new commit
    #[allow(dead_code)]
    pub fn update_branch_ref(
        repo: &Repository,
        branch_name: &str,
        commit: &str,
    ) -> Result<(), RgitError> {
        let branch_path = repo.branches.join(branch_name);

        if !FsOps::path_exists(&branch_path) {
            return Err(RgitError::BranchNotFound(branch_name.to_string()));
        }

        FsOps::create_file_with_content(branch_path.as_path(), commit.as_bytes())?;
        Ok(())
    }

    /// Switch to a tag
    pub fn switch_tag(repo: &Repository, tag_name: &str) -> Result<String, RgitError> {
        let tag_path = repo.tags.as_path().join(tag_name);
        if !FsOps::path_exists(tag_path.as_path()) {
            return Err(RgitError::TagNotFound(tag_name.to_string()));
        }
        let commit = Self::get_branch_commit(tag_path.as_path())?;
        RefOps::write_head_reference(repo, Path::new(""))?;
        Ok(commit)
    }

    /// Switch to a branch
    pub fn switch_branch(repo: &Repository, branch_name: &str) -> Result<String, RgitError> {
        let branch_path = repo.branches.join(branch_name);

        if !FsOps::path_exists(&branch_path) {
            return Err(RgitError::BranchNotFound(branch_name.to_string()));
        }

        let commit = Self::get_branch_commit(&branch_path)?;
        RefOps::write_head_reference(repo, &branch_path)?;

        Ok(commit)
    }

    /// Validate that all required ref directories exist
    #[allow(dead_code)]
    pub fn validate_ref_structure(repo: &Repository) -> Result<(), RgitError> {
        if !FsOps::path_exists(&repo.branches) {
            return Err(RgitError::NotInitialized(String::from("refs/heads")));
        }
        Ok(())
    }
}
