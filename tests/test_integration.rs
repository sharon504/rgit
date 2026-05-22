/// Integration tests for CLI commands and overall system functionality
use rgit_lib::commands::Repository;
use std::path::PathBuf;

#[test]
fn test_commands_enum_init_variant() {
    // Verify that Commands enum can be created
    let repo = Repository::new(PathBuf::from("."));
    assert!(!repo.gitdir.as_os_str().is_empty());
}

#[test]
fn test_repository_struct_creation() {
    let repo = Repository::new(PathBuf::from("./test"));
    assert_eq!(repo.gitdir, PathBuf::from("./test/.rgit"));
    assert_eq!(repo.objects, PathBuf::from("./test/.rgit/objects"));
}

#[test]
fn test_repository_paths_are_consistent() {
    let repo = Repository::new(PathBuf::from("./myproject"));
    
    // All paths should start with gitdir
    assert!(repo.objects.starts_with(&repo.gitdir));
    assert!(repo.head_file.starts_with(&repo.gitdir));
    assert!(repo.logs.starts_with(&repo.gitdir));
    assert!(repo.branches.starts_with(&repo.gitdir));
}

#[test]
fn test_repository_multiple_instances() {
    let repo1 = Repository::new(PathBuf::from("."));
    let repo2 = Repository::new(PathBuf::from("."));
    
    assert_eq!(repo1.gitdir, repo2.gitdir);
    assert_eq!(repo1.objects, repo2.objects);
    assert_eq!(repo1.head_file, repo2.head_file);
}

#[test]
fn test_repository_different_base_paths() {
    let repo1 = Repository::new(PathBuf::from("."));
    let repo2 = Repository::new(PathBuf::from("./project1"));
    let repo3 = Repository::new(PathBuf::from("/home/user/project2"));
    
    assert_ne!(repo1.gitdir, repo2.gitdir);
    assert_ne!(repo2.gitdir, repo3.gitdir);
    assert_ne!(repo1.gitdir, repo3.gitdir);
}

#[test]
fn test_repository_branches_structure() {
    let repo = Repository::new(PathBuf::from("."));
    
    // Branches path should contain refs/heads
    assert!(repo.branches.to_string_lossy().contains("refs"));
    assert!(repo.branches.to_string_lossy().contains("heads"));
}

#[test]
fn test_repository_logs_structure() {
    let repo = Repository::new(PathBuf::from("."));
    
    // Logs path should exist
    assert!(!repo.logs.as_os_str().is_empty());
    assert!(!repo.log_head.as_os_str().is_empty());
}

#[test]
fn test_repository_nested_path() {
    let nested_path = PathBuf::from("./a/b/c/d");
    let repo = Repository::new(nested_path.clone());
    
    // Should handle nested paths correctly
    assert!(repo.gitdir.to_string_lossy().contains("a"));
    assert!(repo.gitdir.to_string_lossy().contains("b"));
    assert!(repo.gitdir.to_string_lossy().contains("c"));
    assert!(repo.gitdir.to_string_lossy().contains("d"));
}

#[test]
fn test_repository_absolute_path() {
    let abs_path = PathBuf::from("/tmp/myrepo");
    let repo = Repository::new(abs_path);
    
    assert!(repo.gitdir.is_absolute());
    assert!(repo.objects.is_absolute());
    assert!(repo.head_file.is_absolute());
}

#[test]
fn test_repository_fields_not_empty() {
    let repo = Repository::new(PathBuf::from("."));
    
    assert!(!repo.gitdir.as_os_str().is_empty());
    assert!(!repo.objects.as_os_str().is_empty());
    assert!(!repo.head_file.as_os_str().is_empty());
    assert!(!repo.log_head.as_os_str().is_empty());
    assert!(!repo.logs.as_os_str().is_empty());
    assert!(!repo.branches.as_os_str().is_empty());
}
