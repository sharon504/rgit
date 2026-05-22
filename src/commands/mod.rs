use std::path::PathBuf;

pub mod constants;
pub mod functions;

#[derive(Debug)]
pub struct Repository {
    pub gitdir: PathBuf,
    pub objects: PathBuf,
    pub head_file: PathBuf,
    pub log_head: PathBuf,
    pub logs: PathBuf,
    pub branches: PathBuf,
    pub current_branch: PathBuf,
}
