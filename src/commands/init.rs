use std::os::unix::ffi::OsStrExt;

use crate::{commands::constants, errors::RgitError};

use super::fs_ops::FsOps;
use crate::commands::Repository;

/// Repository initialization logic
pub struct InitOps;

impl InitOps {
    /// Initialize a new repository with full directory structure
    pub fn init_repo(repo: &mut Repository) -> Result<(), RgitError> {
        // Create .rgit directory
        FsOps::create_dir_safe(repo.gitdir.as_path())?;

        // Create objects directory
        FsOps::create_dir_safe(repo.objects.as_path())?;

        // Create refs/heads
        FsOps::create_dir_all_safe(repo.branches.as_path())?;

        // Create refs/tags
        FsOps::create_dir_all_safe(repo.tags.as_path())?;

        // Create HEAD file pointing to default branch
        let default_branch_path = repo.branches.as_path().join(constants::DEFAULT_BRANCH);
        FsOps::create_file_with_content(
            repo.head_file.as_path(),
            default_branch_path.as_os_str().as_bytes(),
        )?;

        // Update repo state
        repo.current_branch = default_branch_path;

        // Create logs directory structure
        FsOps::create_dir_all_safe(repo.logs.as_path())?;
        let logs_refs_heads = repo.logs.as_path().join("refs/heads");
        FsOps::create_dir_all_safe(logs_refs_heads.as_path())?;

        // Create logs/HEAD file
        FsOps::create_file_with_content(repo.log_head.as_path(), b"")?;

        Ok(())
    }

    /// Validate that an existing repository has all required structure
    pub fn validate_repo_structure(repo: &Repository) -> Result<(), RgitError> {
        let required_paths = vec![
            (&repo.gitdir, ".rgit"),
            (&repo.objects, "objects"),
            (&repo.head_file, "HEAD"),
            (&repo.log_head, "logs/HEAD"),
            (&repo.branches, "refs/heads"),
        ];

        for (path, name) in required_paths {
            if !FsOps::path_exists(path) {
                return Err(RgitError::NotInitialized(name.to_string()));
            }
        }

        Ok(())
    }

    /// Check if a repository already exists
    pub fn repo_exists(repo: &Repository) -> bool {
        FsOps::path_exists(&repo.gitdir)
            && FsOps::path_exists(&repo.objects)
            && FsOps::path_exists(&repo.head_file)
    }
}
