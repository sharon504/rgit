use rgit_lib::errors::RgitError;
use std::io;

#[test]
fn test_error_display_io() {
    let io_err = io::Error::new(io::ErrorKind::Other, "test error");
    let err = RgitError::Io(io_err);
    let display_string = format!("{}", err);
    assert!(display_string.contains("IO Error"));
}

#[test]
fn test_error_display_not_initialized() {
    let err = RgitError::NotInitialized(".rgit".to_string());
    let display_string = format!("{}", err);
    assert!(display_string.contains("Fatal"));
    assert!(display_string.contains(".rgit"));
    assert!(display_string.contains("not found"));
}

#[test]
fn test_error_display_message_file_not_found() {
    let err = RgitError::MessageFileNotFound;
    let display_string = format!("{}", err);
    assert!(display_string.contains("Fatal"));
    assert!(display_string.contains("Message file"));
}

#[test]
fn test_error_display_branch_already_exists() {
    let err = RgitError::BranchAlreadyExists("feature".to_string());
    let display_string = format!("{}", err);
    assert!(display_string.contains("Fatal"));
    assert!(display_string.contains("feature"));
    assert!(display_string.contains("already exists"));
}

#[test]
fn test_error_display_branch_not_found() {
    let err = RgitError::BranchNotFound("develop".to_string());
    let display_string = format!("{}", err);
    assert!(display_string.contains("Fatal"));
    assert!(display_string.contains("develop"));
    assert!(display_string.contains("doesn't exists"));
}

#[test]
fn test_error_display_commit_not_found() {
    let err = RgitError::CommitNotFound;
    let display_string = format!("{}", err);
    assert!(display_string.contains("Fatal"));
    assert!(display_string.contains("commit"));
}

#[test]
fn test_error_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let rgit_err: RgitError = io_err.into();

    match rgit_err {
        RgitError::Io(_) => {
            // Success - error was converted properly
            assert!(true);
        }
        _ => panic!("Expected RgitError::Io variant"),
    }
}

#[test]
fn test_error_debug_output() {
    let err = RgitError::BranchNotFound("test-branch".to_string());
    let debug_string = format!("{:?}", err);
    assert!(debug_string.contains("BranchNotFound"));
    assert!(debug_string.contains("test-branch"));
}

#[test]
fn test_error_is_error_trait() {
    let err = RgitError::MessageFileNotFound;
    let _: &dyn std::error::Error = &err;
    // If this compiles, error implements std::error::Error
    assert!(true);
}

#[test]
fn test_error_clone() {
    let err1 = RgitError::BranchAlreadyExists("feature".to_string());
    // Note: RgitError doesn't implement Clone directly, but we can test
    // that we can construct similar errors
    let err2 = RgitError::BranchAlreadyExists("feature".to_string());

    let display1 = format!("{}", err1);
    let display2 = format!("{}", err2);
    assert_eq!(display1, display2);
}
