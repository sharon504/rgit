use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

/// Filesystem operations helpers
pub struct FsOps;

impl FsOps {
    /// Create a file with initial content
    pub fn create_file_with_content(path: &Path, content: &[u8]) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        file.write_all(content)?;
        Ok(())
    }

    /// Append content to a file
    pub fn append_to_file(path: &Path, content: &[u8]) -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(content)?;
        Ok(())
    }

    /// Read entire file to string
    pub fn read_file(path: &Path) -> std::io::Result<String> {
        fs::read_to_string(path)
    }

    /// Create directory if it doesn't exist
    pub fn create_dir_safe(path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            fs::create_dir(path)?;
        }
        Ok(())
    }

    /// Create directory and all parents
    pub fn create_dir_all_safe(path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// Check if path exists
    pub fn path_exists(path: &Path) -> bool {
        path.exists()
    }

    /// Normalize path by removing leading "./"
    #[allow(dead_code)]
    pub fn normalize_path(path: &Path) -> PathBuf {
        let path_str = path.to_string_lossy();
        PathBuf::from(
            path_str
                .strip_prefix("./")
                .map(|s| s.to_string())
                .unwrap_or_else(|| path_str.to_string()),
        )
    }

    /// Remove directory and all contents
    pub fn remove_dir_safe(path: &Path) -> std::io::Result<()> {
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    /// Recursively copy directory
    pub fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
        fs::create_dir_all(src)?;
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                FsOps::copy_dir_recursive(&entry.path(), &dest.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dest.join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
