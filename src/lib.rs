use anyhow::{Error, Result};
use std::path::Path;
use url::Url;

pub type PathThatMustBeAbsolute = Path;
pub type PathAsUriAsString = String;

/// Converts a path to a URI.
/// The path must be absolute.
///
/// This is a preferred way if you want to have Url type and handle errors.
pub fn get_path_as_uri_or_error(path: &PathThatMustBeAbsolute) -> Result<Url> {
    match Url::from_file_path(path) {
        Ok(url) => Ok(url),
        Err(_) => {
            let msg = format!(
                "Failed to convert path to URI. Maybe it is not absolute. Path: '{path:?}'."
            );
            Err(Error::msg(msg))
        }
    }
}

/// Converts a path to a URI and returns it as a string.
/// The path must be absolute.
///
/// This is a preferred way if you want to handle errors, but you need a string.
///
/// Example:
/// ```rust
/// use std::path::Path;
/// use path_as_uri::get_path_as_uri_or_error;
///
/// let path = Path::new("/path/to/file.txt");
/// let uri = get_path_as_uri_or_error(path).unwrap();
/// assert_eq!(uri.to_string(), "file:///path/to/file.txt");
/// ```
pub fn get_path_as_uri_string_or_error(path: &PathThatMustBeAbsolute) -> Result<PathAsUriAsString> {
    // Just convert the Url to a string, if it is valid. Return the same error if it is not.
    get_path_as_uri_or_error(path).map(|url| url.to_string())
}

/// Converts a path to a URI and returns it as a string.
/// The path must be absolute.
///
/// This is a preferred way if you are optimistic and want unrecoverable error on failure.
pub fn get_path_as_uri_string(path: &PathThatMustBeAbsolute) -> PathAsUriAsString {
    get_path_as_uri_string_or_error(path).unwrap()
}

#[cfg(test)]
mod tests;
