use std::fs;
use std::path::Path;
use file_system::{get_dir_path, get_file_contents};

#[test]
pub fn test_get_dir() {
    // TODO: make returing Result<> and check all possible outcomes
    get_dir_path("test_dir");

    fs::remove_dir(Path::new("test_dir")).unwrap();
}

#[test]
pub fn test_get_file_contents() {
    // TODO: make returing Result<> and check all possible outcomes
    assert_eq!("Approvals!\n", get_file_contents("contents", Path::new("src/tests")));
}

// TODO: Write tests for write_actual
