use rgit_lib::core::utils::Utils;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_copy_dir_creates_dest() {
    let temp = TempDir::new().unwrap();
    let src = temp.path().join("src");
    let dest = temp.path().join("dest");

    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("test.txt"), "content").unwrap();

    Utils::copy_dir(&src, &dest).unwrap();

    assert!(dest.exists());
    assert!(dest.join("test.txt").exists());
    let content = fs::read_to_string(dest.join("test.txt")).unwrap();
    assert_eq!(content, "content");
}

#[test]
fn test_copy_dir_recursive() {
    let temp = TempDir::new().unwrap();
    let src = temp.path().join("src");
    let dest = temp.path().join("dest");

    // Create nested structure
    fs::create_dir_all(&src.join("subdir/nested")).unwrap();
    fs::write(src.join("file1.txt"), "content1").unwrap();
    fs::write(src.join("subdir/file2.txt"), "content2").unwrap();
    fs::write(src.join("subdir/nested/file3.txt"), "content3").unwrap();

    Utils::copy_dir(&src, &dest).unwrap();

    assert!(dest.join("file1.txt").exists());
    assert!(dest.join("subdir/file2.txt").exists());
    assert!(dest.join("subdir/nested/file3.txt").exists());

    assert_eq!(
        fs::read_to_string(dest.join("subdir/nested/file3.txt")).unwrap(),
        "content3"
    );
}

#[test]
fn test_copy_dir_empty() {
    let temp = TempDir::new().unwrap();
    let src = temp.path().join("src");
    let dest = temp.path().join("dest");

    fs::create_dir_all(&src).unwrap();

    Utils::copy_dir(&src, &dest).unwrap();

    assert!(dest.exists());
}

#[test]
fn test_next_snapshot_index_no_objects_dir() {
    let temp = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    // Change to temp directory
    if let Err(_) = std::env::set_current_dir(&temp) {
        std::env::set_current_dir(original_dir).unwrap();
        return;
    }

    let index = Utils::next_snapshot_index().unwrap();

    // Restore original directory before assertion
    std::env::set_current_dir(original_dir).unwrap();

    assert_eq!(index, 0);
}

#[test]
fn test_next_snapshot_index_existing_snapshots() {
    let temp = TempDir::new().unwrap();
    let snapshots = temp.path().join(".rgit/objects");
    fs::create_dir_all(&snapshots).unwrap();

    fs::create_dir_all(snapshots.join("snap-0")).unwrap();
    fs::create_dir_all(snapshots.join("snap-1")).unwrap();
    fs::create_dir_all(snapshots.join("snap-2")).unwrap();

    let original_dir = std::env::current_dir().unwrap();

    // Change to temp directory
    if let Err(_) = std::env::set_current_dir(&temp) {
        std::env::set_current_dir(original_dir).unwrap();
        return;
    }

    let index = Utils::next_snapshot_index().unwrap();

    // Restore original directory before assertion
    std::env::set_current_dir(original_dir).unwrap();

    assert_eq!(index, 3);
}
