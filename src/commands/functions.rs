use crate::{
    commands::{Repository, constants},
    core::utils::Utils,
    errors::RgitError,
};
use chrono::{Local, TimeZone};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

impl Repository {
    pub fn new(path: PathBuf) -> Self {
        let root = path.join(".rgit");
        Self {
            gitdir: root.clone(),
            objects: root.clone().join("objects"),
            head_file: root.clone().join("HEAD"),
            log_head: root.clone().join("logs/HEAD"),
            logs: root.clone().join("logs/"),
            branches: root.clone().join("refs/heads/"),
            current_branch: PathBuf::new(),
        }
    }

    fn get_preexisting_repo(&mut self) -> Result<(), RgitError> {
        if !self.gitdir.exists() {
            return Err(RgitError::NotInitialized(String::from(".rgit")));
        }
        if !self.objects.exists() {
            return Err(RgitError::NotInitialized(String::from("objects")));
        }
        if !self.head_file.exists() {
            return Err(RgitError::NotInitialized(String::from("HEAD")));
        }
        if !self.log_head.exists() {
            return Err(RgitError::NotInitialized(String::from("logs/HEAD")));
        }
        if !self.branches.exists() {
            return Err(RgitError::NotInitialized(String::from("refs/heads")));
        }
        let current_branch = fs::read_to_string(self.head_file.as_path())?;
        self.current_branch = PathBuf::from(current_branch);

        Ok(())
    }

    pub fn find(&mut self) -> Result<(), RgitError> {
        self.get_preexisting_repo()?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), RgitError> {
        match self.find() {
            Ok(_) => {
                println!("Repository already exists");
            }
            Err(_) => {
                // Create .git directory
                fs::create_dir(&self.gitdir)?;

                // Create objects directory
                fs::create_dir(self.objects.as_path())?;

                // Create refs directory
                let refs_dir = self.gitdir.as_path().join("refs/");
                fs::create_dir(refs_dir.as_path())?;

                // Create refs/heads
                fs::create_dir(self.branches.as_path())?;

                // Create HEAD
                let mut head = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(self.head_file.as_path())?;

                let _ = head.write(
                    self.branches
                        .as_path()
                        .join(constants::DEFAULT_BRANCH)
                        .as_os_str()
                        .as_bytes(),
                )?;
                let current_branch = fs::read_to_string(self.head_file.as_path())?;
                self.current_branch = PathBuf::from(current_branch);

                // Create logs directory
                let logs_dir = self.gitdir.as_path().join("logs/");
                fs::create_dir(logs_dir.as_path())?;

                // Create logs/refs/heads
                let logs_refs_head = logs_dir.join("refs/heads");
                fs::create_dir_all(logs_refs_head.as_path())?;

                // Create logs/HEAD
                let logs_head = logs_dir.join("HEAD");
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(logs_head)?;

                println!("Repository initialized");
            }
        }
        Ok(())
    }

    pub fn add(&self, files: Vec<String>) -> Result<(), RgitError> {
        println!("Adding files: {:?}", files);
        Ok(())
    }

    pub fn commit(&self, message: String) -> Result<(), RgitError> {
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
        Utils::write_commit(child, parent, timestamp, message, self)?;
        Ok(())
    }

    pub fn log(&self) -> Result<(), RgitError> {
        if !self.head_file.exists() {
            return Err(RgitError::MessageFileNotFound);
        }
        let lines = fs::read_to_string(self.head_file.as_path())?;
        for line in lines.lines().rev() {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split("\t").collect();
            if parts.len() >= 3 {
                let datetime = Local
                    .timestamp_opt(parts[0].parse().unwrap_or(0), 0)
                    .unwrap()
                    .format("%b %e %Y")
                    .to_string();

                println!(
                    "Snapshot:\t{}\nDate:\t{}\nMessage:\t{}",
                    parts[1], datetime, parts[2]
                );
            }
        }
        Ok(())
    }

    pub fn branch(&self, name: String) -> Result<(), RgitError> {
        let target_branch = self.branches.join(name.clone());
        if target_branch.exists() {
            return Err(RgitError::BranchAlreadyExists(name));
        }

        let mut branch_ref = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(target_branch.as_path())?;

        let mut logs_head = OpenOptions::new()
            .create(false)
            .append(true)
            .open(self.log_head.as_path())?;
        logs_head.write_all(format!("Created branch: {name}").as_bytes())?;

        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.logs.as_path().join(format!("refs/heads/{name}")))?;

        let current_commit = fs::read_to_string(self.current_branch.as_path())?;
        branch_ref.write_all(current_commit.as_bytes())?;

        println!("Created branch: {name}");
        Ok(())
    }

    pub fn checkout(&self, name: String) -> Result<(), RgitError> {
        let target_branch = self.branches.join(name.clone());
        if !target_branch.exists() {
            return Err(RgitError::BranchNotFound(name));
        }

        let snap_name = fs::read_to_string(target_branch.as_path())?;
        let snap_name = snap_name.trim();

        let workdir = Path::new(constants::WORKDIR);
        if !snap_name.is_empty() {
            let snap = self.objects.as_path().join(snap_name);
            if !snap.exists() {
                return Err(RgitError::CommitNotFound);
            }
            fs::remove_dir_all(workdir)?;
            Utils::copy_dir(snap.as_path(), workdir)?;
        }

        let mut head = OpenOptions::new()
            .create(false)
            .write(true)
            .truncate(true)
            .open(self.head_file.as_path())?;
        head.write_all(target_branch.as_path().as_os_str().as_bytes())?;

        let mut logs_head = OpenOptions::new()
            .create(false)
            .append(true)
            .open(self.log_head.as_path())?;
        logs_head.write_all(format!("Checkout branch: {name}").as_bytes())?;

        Ok(())
    }
}
