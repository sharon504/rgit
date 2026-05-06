use crate::errors::RgitError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Repository {
    pub gitdir: PathBuf,
}

impl Repository {
    pub fn new(path: PathBuf) -> Self {
        Self {
            gitdir: path.join(".rgit"),
            // workdir: path,
        }
    }

    pub fn find() -> Result<Self, RgitError> {
        let path = Path::new(".").join(".rgit");
        if path.is_dir() {
            return Ok(Self { gitdir: path });
        }
        Err(RgitError::NotInitialized)
    }

    pub fn commit(&self, message: String) -> Result<(), RgitError> {
        println!("commit: {}", message);
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

    pub fn add(&self, files: Vec<String>) -> Result<(), RgitError> {
        println!("Adding files: {:?}", files);
        Ok(())
    }
}
