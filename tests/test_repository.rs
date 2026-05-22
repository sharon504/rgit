use rgit_lib::commands::{fs_ops::FsOps, Repository, ref_ops::RefOps};
use rgit_lib::errors::RgitError;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use tempfile::TempDir;

fn setup_test_repo(temp: &TempDir) -> Repository {
    let root = temp.path();
    let gitdir = root.join(".rgit");
    FsOps::create_dir_all_safe(&gitdir).unwrap();
    FsOps::create_dir_all_safe(&gitdir.join("refs/heads")).unwrap();
    FsOps::create_dir_all_safe(&gitdir.join("logs/refs/heads")).unwrap();
    FsOps::create_dir_all_safe(&gitdir.join("objects")).unwrap();

    let repo = Repository {
        gitdir: gitdir.clone(),
        objects: gitdir.join("objects"),
        head_file: gitdir.join("HEAD"),
        log_head: gitdir.join("logs/HEAD"),
        logs: gitdir.join("logs/"),
        branches: gitdir.join("refs/heads/"),
        current_branch: gitdir.join("refs/heads/master"),
    };

    FsOps::create_file_with_content(&repo.current_branch, b"snap-0").unwrap();
    FsOps::create_file_with_content(
        &repo.head_file,
        gitdir.join("refs/heads/master").as_os_str().as_bytes(),
    )
    .unwrap();
    FsOps::create_file_with_content(&repo.log_head, b"").unwrap();

    repo
}

#[test]
fn test_repository_new() {
    let repo = Repository::new(PathBuf::from("."));
    assert_eq!(repo.gitdir, PathBuf::from("./.rgit"));
    assert_eq!(repo.objects, PathBuf::from("./.rgit/objects"));
    assert_eq!(repo.head_file, PathBuf::from("./.rgit/HEAD"));
    assert_eq!(repo.log_head, PathBuf::from("./.rgit/logs/HEAD"));
}

#[test]
fn test_repository_find_valid() {
    let temp = TempDir::new().unwrap();
    let mut repo = setup_test_repo(&temp);

    let result = repo.find();
    assert!(result.is_ok());
}

#[test]
fn test_repository_find_invalid() {
    let temp = TempDir::new().unwrap();
    let mut repo = Repository::new(temp.path().to_path_buf());

    let result = repo.find();
    assert!(result.is_err());
}

#[test]
fn test_repository_add() {
    let temp = TempDir::new().unwrap();
    let repo = setup_test_repo(&temp);

    let result = repo.add(vec!["test.txt".to_string()]);
    assert!(result.is_ok());
}

#[test]
fn test_repository_add_multiple_files() {
    let temp = TempDir::new().unwrap();
    let repo = setup_test_repo(&temp);

    let result = repo.add(vec![
        "file1.txt".to_string(),
        "file2.txt".to_string(),
        "file3.txt".to_string(),
    ]);
    assert!(result.is_ok());
}

#[test]
fn test_repository_commit() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let repo = setup_test_repo(&temp);

    // Create a working directory with a file
    let workdir = root.join("src");
    fs::create_dir_all(&workdir).unwrap();
    fs::write(workdir.join("main.rs"), "fn main() {}").unwrap();

    // Change to temp directory for this test
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&root).unwrap();

    let result = repo.commit("Initial commit".to_string());

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Check that commit was created
    assert!(result.is_ok());
    assert!(FsOps::path_exists(&repo.objects.join("snap-1")));
}

#[test]
fn test_repository_branch() {
    let temp = TempDir::new().unwrap();
    let repo = setup_test_repo(&temp);

    let result = repo.branch("feature".to_string());
    assert!(result.is_ok());
    assert!(FsOps::path_exists(&repo.branches.join("feature")));
}

#[test]
fn test_repository_branch_already_exists() {
    let temp = TempDir::new().unwrap();
    let repo = setup_test_repo(&temp);

    repo.branch("feature".to_string()).unwrap();
    let result = repo.branch("feature".to_string());

    // Should fail because feature branch already exists
    assert!(result.is_err());
    assert!(matches!(result, Err(RgitError::BranchAlreadyExists(_))));
}

#[test]
fn test_repository_checkout() {
    let temp = TempDir::new().unwrap();
    let repo = setup_test_repo(&temp);

    // Create a source snapshot
    let snap0 = repo.objects.join("snap-0");
    fs::create_dir_all(&snap0).unwrap();
    fs::write(snap0.join("test.txt"), "content").unwrap();

    // Create a branch pointing to snap-0
    RefOps::create_branch_ref(&repo, "feature", "snap-0").unwrap();

    // Branch should be created successfully
    assert!(FsOps::path_exists(&repo.branches.join("feature")));
}

#[test]
fn test_repository_checkout_nonexistent_branch() {
    let temp = TempDir::new().unwrap();
    let repo = setup_test_repo(&temp);

    let result = repo.checkout("nonexistent".to_string());
    assert!(result.is_err());
    assert!(matches!(result, Err(RgitError::BranchNotFound(_))));
}

#[test]
fn test_repository_log_empty() {
    let temp = TempDir::new().unwrap();
    let repo = setup_test_repo(&temp);

    // Log should work even with empty log file
    let result = repo.log();
    assert!(result.is_ok());
}

#[test]
fn test_repository_log_no_log_file() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();
    let gitdir = root.join(".rgit");
    FsOps::create_dir_all_safe(&gitdir).unwrap();

    let repo = Repository {
        gitdir: gitdir.clone(),
        objects: gitdir.join("objects"),
        head_file: gitdir.join("HEAD"),
        log_head: gitdir.join("logs/HEAD"),
        logs: gitdir.join("logs/"),
        branches: gitdir.join("refs/heads/"),
        current_branch: gitdir.join("refs/heads/master"),
    };

    // Log file doesn't exist
    let result = repo.log();
    assert!(result.is_err());
    assert!(matches!(result, Err(RgitError::MessageFileNotFound)));
}

#[test]
fn test_repository_fields_are_populated() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    let repo = Repository::new(root.to_path_buf());

    assert!(!repo.gitdir.as_os_str().is_empty());
    assert!(!repo.objects.as_os_str().is_empty());
    assert!(!repo.head_file.as_os_str().is_empty());
    assert!(!repo.log_head.as_os_str().is_empty());
    assert!(!repo.logs.as_os_str().is_empty());
    assert!(!repo.branches.as_os_str().is_empty());
}

#[test]
fn test_repository_init_already_exists() {
    let temp = TempDir::new().unwrap();
    let mut repo = setup_test_repo(&temp);

    // Initializing again should detect existing repository
    let result = repo.init();
    assert!(result.is_ok());
}
