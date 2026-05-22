use crate::{
    commands::{
        fs_ops::FsOps,
        init::InitOps,
        log_ops::LogOps,
        ref_ops::{CheckoutTarget, RefOps},
    },
    core::utils::Utils,
    errors::RgitError,
};
use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub mod constants;
pub mod fs_ops;
pub mod init;
pub mod log_ops;
pub mod ref_ops;

#[derive(Debug)]
pub struct Repository {
    pub gitdir: PathBuf,
    pub objects: PathBuf,
    pub head_file: PathBuf,
    pub log_head: PathBuf,
    pub logs: PathBuf,
    pub branches: PathBuf,
    pub tags: PathBuf,
    pub current_branch: PathBuf,
}

impl Repository {
    /// Create a new repository instance
    pub fn new(path: PathBuf) -> Self {
        let root = path.join(".rgit");
        Self {
            gitdir: root.clone(),
            objects: root.clone().join("objects"),
            head_file: root.clone().join("HEAD"),
            log_head: root.clone().join("logs/HEAD"),
            logs: root.clone().join("logs/"),
            branches: root.clone().join("refs/heads/"),
            tags: root.clone().join("refs/tags/"),
            current_branch: PathBuf::new(),
        }
    }

    /// Find and validate existing repository
    pub fn find(&mut self) -> Result<(), RgitError> {
        InitOps::validate_repo_structure(self)?;
        RefOps::read_current_branch(self)?;
        Ok(())
    }

    /// Initialize a new repository
    pub fn init(&mut self) -> Result<(), RgitError> {
        match self.find() {
            Ok(_) => {
                println!("Repository already exists");
            }
            Err(_) => {
                InitOps::init_repo(self)?;
                println!("Repository initialized");
            }
        }
        Ok(())
    }

    /// Add files to staging (placeholder)
    pub fn add(&self, files: Vec<String>) -> Result<(), RgitError> {
        println!("Adding files: {:?}", files);
        Ok(())
    }

    /// Create a new commit (snapshot)
    pub fn commit(&self, message: String) -> Result<(), RgitError> {
        if self.current_branch.as_path().as_os_str().is_empty() {
            return Err(RgitError::NotInBranch);
        }
        let n = Utils::next_snapshot_index()? + 1;
        Utils::copy_dir(
            Path::new(constants::WORKDIR),
            self.objects.join(format!("snap-{}", n)).as_path(),
        )?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let parent = format!("snap-{}", n - 1);
        let child = format!("snap-{}", n);

        LogOps::write_commit_log(child.clone(), parent, timestamp, message, self)?;

        // Update branch pointer to point to new commit
        let normalized_branch = FsOps::normalize_path(&self.current_branch);

        // Ensure parent directory for branch file exists
        if let Some(parent_dir) = normalized_branch.parent() {
            FsOps::create_dir_safe(parent_dir)?;
        }

        FsOps::create_file_with_content(normalized_branch.as_path(), child.as_bytes())?;

        Ok(())
    }

    /// Display commit history
    pub fn log(&self) -> Result<(), RgitError> {
        if !FsOps::path_exists(self.log_head.as_path()) {
            return Err(RgitError::MessageFileNotFound);
        }

        let entries = LogOps::read_log_entries(self.log_head.as_path())?;

        for entry in entries.iter().rev() {
            let datetime = LogOps::format_timestamp(entry.timestamp);
            println!(
                "Snapshot:\t{}\nDate:\t{}\nMessage:\t{}",
                entry.snapshot, datetime, entry.message
            );
        }

        Ok(())
    }

    /// Create a new branch
    pub fn branch(&self, name: String) -> Result<(), RgitError> {
        // Get current commit that we're pointing to
        let current_commit = FsOps::read_file(self.current_branch.as_path())?;
        let current_commit = current_commit.trim();

        // Create branch reference
        RefOps::create_branch_ref(self, &name, current_commit)?;

        // Log the action
        LogOps::write_action_log(&format!("Created branch: {}\n", name), self)?;

        println!("Created branch: {name}");
        Ok(())
    }

    /// Checkout a branch
    pub fn checkout(&self, name: String) -> Result<(), RgitError> {
        let snap_name = match RefOps::resolve_checkout_target(self, &name)? {
            CheckoutTarget::Branch(branch) => {
                LogOps::write_action_log(&format!("Checkout branch: {}\n", branch), self)?;
                RefOps::switch_branch(self, &branch)?
            }
            CheckoutTarget::Tag(tag) => {
                LogOps::write_action_log(&format!("Checkout tag: {}\n", tag), self)?;
                RefOps::switch_tag(self, &tag)?
            }
        };
        let snap_name = snap_name.trim();

        // Update working directory if commit is not empty
        if !snap_name.is_empty() {
            let snap = self.objects.as_path().join(snap_name);
            if !FsOps::path_exists(snap.as_path()) {
                return Err(RgitError::CommitNotFound);
            }
            FsOps::remove_dir_safe(Path::new(constants::WORKDIR))?;
            FsOps::copy_dir_recursive(snap.as_path(), Path::new(constants::WORKDIR))?;
        }
        Ok(())
    }

    /// Tag a commit (immutable)
    pub fn tag(&self, name: String) -> Result<(), RgitError> {
        let tag_dir = self.tags.as_path().join(&name);
        let current_commit = FsOps::read_file(self.current_branch.as_path())?;
        if tag_dir.exists() {
            return Err(RgitError::TagAlreadyExists(name));
        }
        FsOps::create_file_with_content(tag_dir.as_path(), current_commit.as_bytes())?;
        Ok(())
    }
}
