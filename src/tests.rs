use super::*;
use std::panic;

// Absolute and relative paths differ per platform: a Unix path has no drive on Windows.
#[cfg(unix)]
mod platform {
    pub const ABSOLUTE: &str = "/path/to/file.txt";
    pub const ABSOLUTE_URI: &str = "file:///path/to/file.txt";
    pub const WITH_SPACES: &str = "/path/to/file with spaces.txt";
    pub const WITH_SPACES_URI: &str = "file:///path/to/file%20with%20spaces.txt";
}

#[cfg(windows)]
mod platform {
    pub const ABSOLUTE: &str = r"C:\path\to\file.txt";
    pub const ABSOLUTE_URI: &str = "file:///C:/path/to/file.txt";
    pub const WITH_SPACES: &str = r"C:\path\to\file with spaces.txt";
    pub const WITH_SPACES_URI: &str = "file:///C:/path/to/file%20with%20spaces.txt";
}

const RELATIVE: &str = "./invalid/path/to/file.txt";

#[test]
fn test_get_path_as_uri() {
    let uri = get_path_as_uri_or_error(Path::new(platform::ABSOLUTE)).unwrap();
    assert_eq!(uri.as_str(), platform::ABSOLUTE_URI);
}

#[test]
fn test_get_path_as_uri_as_string() {
    let uri_string = get_path_as_uri_string_or_error(Path::new(platform::ABSOLUTE)).unwrap();
    assert_eq!(uri_string, platform::ABSOLUTE_URI);
}

#[test]
fn test_get_path_as_uri_invalid() {
    let result = get_path_as_uri_or_error(Path::new(RELATIVE));
    assert!(matches!(result, Err(PathAsUriError::NotAbsolute { .. })));
}

#[test]
fn test_error_message_contains_the_path() {
    let error = get_path_as_uri_or_error(Path::new(RELATIVE)).unwrap_err();
    assert!(
        matches!(&error, PathAsUriError::NotAbsolute { path } if path == Path::new(RELATIVE)),
        "{error:?}"
    );
    assert!(error.to_string().contains(RELATIVE), "{error}");
}

#[test]
fn test_get_path_as_uri_string() {
    let uri_string = get_path_as_uri_string(Path::new(platform::ABSOLUTE));
    assert_eq!(uri_string, platform::ABSOLUTE_URI);
}

#[test]
fn test_get_path_as_uri_string_invalid() {
    let result = panic::catch_unwind(|| get_path_as_uri_string(Path::new(RELATIVE)));
    assert!(result.is_err());
}

#[test]
fn test_get_path_as_uri_string_whitespace() {
    let uri_string = get_path_as_uri_string(Path::new(platform::WITH_SPACES));
    assert_eq!(uri_string, platform::WITH_SPACES_URI);
}
