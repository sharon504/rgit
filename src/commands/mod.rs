use crate::core::utils::Utils;
use crate::errors::RgitError;
use chrono::{Local, TimeZone};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct Repository {
    pub gitdir: PathBuf,
    pub snapshots: PathBuf,
    pub message_file: PathBuf,
}

impl Repository {
    pub fn new(path: PathBuf) -> Self {
        Self {
            gitdir: path.join(".rgit"),
            snapshots: path.join(".rgit").join("snapshots"),
            message_file: path.join(".rgit").join("messages"),
        }
    }

    pub fn find() -> Result<Self, RgitError> {
        let path = Path::new(".").join(".rgit");
        if path.is_dir() {
            return Ok(Repository::new(PathBuf::from(".")));
        }
        Err(RgitError::NotInitialized)
    }

    pub fn commit(&self, message: String) -> Result<(), RgitError> {
        let n = Utils::next_snapshot_index()?;
        Utils::copy_dir(
            Path::new("workdir"),
            self.snapshots.join(format!("snap-{}", n)).as_path(),
        )?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // let timestamp = chrono::Local::now().to_rfc2822();
        let content = format!("{}\tsnap-{}\t{}\n", timestamp, n, message);
        Utils::write(content, self.message_file.as_path())?;
        Ok(())
    }

    pub fn init(&self) -> Result<(), RgitError> {
        match Repository::find() {
            Ok(_) => {
                println!("Repository already exists");
            }
            Err(_) => {
                let folder = fs::create_dir(&self.gitdir);
                if let Err(e) = folder {
                    return Err(RgitError::RootFolderInstallation(Box::new(e)));
                }
                let snapshots_dir = self.gitdir.as_path().join("snapshots/");
                let snapshots = fs::create_dir(&snapshots_dir);
                if let Err(e) = snapshots {
                    return Err(RgitError::CreatingFolder(snapshots_dir, Box::new(e)));
                }
                println!("repo initialized");
            }
        }
        Ok(())
    }

    pub fn log(&self) -> Result<(), RgitError> {
        if !self.message_file.exists() {
            return Err(RgitError::MessageFileNotFound);
        }
        let lines = fs::read_to_string(self.message_file.as_path())?;
        for line in lines.lines().rev() {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split("\t").collect();
            if parts.len() >= 3 {
                let timestamp_str = parts[0];
                let snapshot = parts[1];
                let message = parts[2];
                let timestamp_secs: i64 = timestamp_str.parse().unwrap_or(0);

                let datetime = Local.timestamp_opt(timestamp_secs, 0).unwrap();
                let formatted_date = datetime.format("%b %e %Y").to_string();

                println!("Snapshot: {}", snapshot);
                println!("Date    : {}", formatted_date);
                println!("Message : {}", message);
                println!();
            }
        }
        Ok(())
    }

    pub fn add(&self, files: Vec<String>) -> Result<(), RgitError> {
        println!("Adding files: {:?}", files);
        Ok(())
    }
}
