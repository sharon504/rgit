pub struct Utils {}

use std::fs;
use std::io::Write;
use std::path::Path;

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

    pub fn write(content: String, dest: &Path) -> std::io::Result<()> {
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(dest)?;
        f.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn next_snapshot_index() -> std::io::Result<usize> {
        let snapshots = Path::new(".rgit/snapshots");
        if !snapshots.exists() {
            fs::create_dir_all(snapshots)?;
            return Ok(0);
        }
        let count = fs::read_dir(snapshots)?.count();
        Ok(count)
    }
}
