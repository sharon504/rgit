use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use crate::commands::Repository;

pub struct Utils {}

impl Utils {
    // pub fn new() -> Self {
    //     Utils {}
    // }
    pub fn copy_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
        fs::create_dir_all(src)?;
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                Utils::copy_dir(&entry.path(), &dest.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dest.join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    pub fn write_commit(
        child: String,
        parent: String,
        timestamp: u64,
        message: String,
        repo: &Repository,
    ) -> std::io::Result<()> {
        let branch = repo.current_branch.strip_prefix(repo.gitdir.as_path());
        let content = format!(
            "{}\t{}\t{}\t{}\tcommit\n",
            child, parent, timestamp, message
        );

        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(repo.log_head.as_path())?;
        f.write_all(content.as_bytes())?;

        let mut logs_ref = fs::OpenOptions::new().append(true).create(true).open(
            repo.logs
                .as_path()
                .join(branch.unwrap_or(PathBuf::from("refs/heads/").as_path())),
        )?;
        logs_ref.write_all(content.as_bytes())?;

        let mut branch_head = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(repo.current_branch.as_path())?;
        branch_head.write_all(child.as_bytes())?;

        Ok(())
    }

    pub fn next_snapshot_index() -> std::io::Result<usize> {
        let snapshots = Path::new(".rgit/objects");
        if !snapshots.exists() {
            fs::create_dir_all(snapshots)?;
            return Ok(0);
        }
        let count = fs::read_dir(snapshots)?.count();
        Ok(count)
    }
}
