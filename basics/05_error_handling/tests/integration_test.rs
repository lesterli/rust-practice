use std::fs;
use std::path::PathBuf;

use error_handling::{parse_number, read_username_from_file, safe_divide};

#[test]
fn integration_parse_and_divide() {
    assert_eq!(parse_number("8").unwrap(), 8);
    assert!(safe_divide(4, 2).is_ok());
    assert!(safe_divide(4, 0).is_err());
}

#[test]
fn integration_read_username() {
    let mut path: PathBuf = std::env::temp_dir();
    path.push("rust_integration_user.txt");
    fs::write(&path, "integration_user\n").expect("write tmp");
    let got = read_username_from_file(path.to_str().unwrap()).expect("read file");
    assert_eq!(got, "integration_user");
    let _ = fs::remove_file(path);
}