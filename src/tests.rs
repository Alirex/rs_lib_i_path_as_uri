use super::*;
use std::panic;

#[test]
fn test_get_path_as_uri() {
    let path_as_string = "/path/to/file.txt";
    let path = Path::new(&path_as_string);
    let uri = get_path_as_uri_or_error(path).unwrap();
    assert_eq!(uri.to_string(), "file:///path/to/file.txt");
}

#[test]
fn test_get_path_as_uri_as_string() {
    let path_as_string = "/path/to/file.txt";
    let path = Path::new(&path_as_string);
    let uri_string = get_path_as_uri_string_or_error(path).unwrap();
    assert_eq!(uri_string, "file:///path/to/file.txt");
}

#[test]
fn test_get_path_as_uri_invalid() {
    let path_as_string = "./invalid/path/to/file.txt";
    let path = Path::new(&path_as_string);
    let result = get_path_as_uri_or_error(path);
    assert!(result.is_err());
}

#[test]
fn test_get_path_as_uri_string() {
    let path_as_string = "/path/to/file.txt";
    let path = Path::new(&path_as_string);
    let uri_string = get_path_as_uri_string(path);
    assert_eq!(uri_string, "file:///path/to/file.txt");
}

#[test]
fn test_get_path_as_uri_string_invalid() {
    let path_as_string = "./invalid/path/to/file.txt";
    let path = Path::new(&path_as_string);
    let result = panic::catch_unwind(|| get_path_as_uri_string(path));
    assert!(result.is_err());
}

#[test]
fn test_get_path_as_uri_string_whitespace() {
    let path_as_string = "/path/to/file with spaces.txt";
    let path = Path::new(&path_as_string);
    let uri_string = get_path_as_uri_string(path);
    assert_eq!(uri_string, "file:///path/to/file%20with%20spaces.txt");
}
