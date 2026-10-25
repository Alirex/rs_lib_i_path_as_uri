//! Convert a path to a `file://` URI.
//!
//! A URI printed in the terminal is clickable in most terminals, so an app can show a path the user opens with one
//! click. The crate is a thin wrapper around [`url::Url::from_file_path`] that percent-encodes the path (spaces and
//! other special characters) and turns the failure into an error with the path in the message.
//!
//! The path must be absolute.
//!
//! ```rust
//! # #[cfg(unix)]
//! # {
//! use std::path::{Path, PathBuf};
//!
//! let uri = path_as_uri::get_path_as_uri_string(Path::new("/path/to/file with spaces.txt"));
//! assert_eq!(uri, "file:///path/to/file%20with%20spaces.txt");
//! # }
//! ```
//!
//! Functions:
//!
//! - [`get_path_as_uri_or_error`]: a [`Url`] or an error.
//! - [`get_path_as_uri_string_or_error`]: a [`String`] or an error.
//! - [`get_path_as_uri_string`]: a [`String`], panics on a relative path.

use std::path::{Path, PathBuf};
use thiserror::Error;
use url::Url;

/// A path that must be absolute (a relative path has no URI).
pub type PathThatMustBeAbsolute = Path;

/// Errors that can occur when converting a path to a `file://` URI.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PathAsUriError {
    /// The path is not absolute, or cannot be represented as a `file://` URI on this platform.
    ///
    /// `Url::from_file_path` rejects relative paths and, on Windows, paths longer than
    /// 260 characters or containing null bytes.
    #[error("Path is not absolute or cannot be represented as a URI: '{}'", path.display())]
    NotAbsolute {
        /// The path that could not be converted.
        path: PathBuf,
    },
}

/// A URI as a string, for example `file:///path/to/file.txt`.
pub type PathAsUriAsString = String;

/// Converts a path to a URI.
/// The path must be absolute.
///
/// This is a preferred way if you want to have [`Url`] type and handle errors.
///
/// # Errors
///
/// Returns [`PathAsUriError::NotAbsolute`] if the path is not absolute or cannot be
/// represented as a `file://` URI on this platform.
///
/// # Examples
///
/// ```rust
/// # #[cfg(unix)]
/// # {
/// use std::path::{Path, PathBuf};
/// use path_as_uri::get_path_as_uri_or_error;
///
/// let uri = get_path_as_uri_or_error(Path::new("/path/to/file.txt")).unwrap();
/// assert_eq!(uri.as_str(), "file:///path/to/file.txt");
///
/// assert!(get_path_as_uri_or_error(Path::new("relative/file.txt")).is_err());
/// # }
/// ```
pub fn get_path_as_uri_or_error(
    path: &PathThatMustBeAbsolute,
) -> std::result::Result<Url, PathAsUriError> {
    Url::from_file_path(path).map_err(|()| PathAsUriError::NotAbsolute {
        path: path.to_path_buf(),
    })
}

/// Converts a path to a URI and returns it as a string.
/// The path must be absolute.
///
/// This is a preferred way if you want to handle errors, but you need a string.
///
/// # Errors
///
/// Returns [`PathAsUriError::NotAbsolute`] if the path is not absolute or cannot be
/// represented as a `file://` URI on this platform. See [`get_path_as_uri_or_error`] for details.
///
/// # Examples
///
/// ```rust
/// # #[cfg(unix)]
/// # {
/// use std::path::{Path, PathBuf};
/// use path_as_uri::get_path_as_uri_string_or_error;
///
/// let uri = get_path_as_uri_string_or_error(Path::new("/path/to/file.txt")).unwrap();
/// assert_eq!(uri, "file:///path/to/file.txt");
/// # }
/// ```
pub fn get_path_as_uri_string_or_error(
    path: &PathThatMustBeAbsolute,
) -> std::result::Result<PathAsUriAsString, PathAsUriError> {
    // Just convert the Url to a string, if it is valid. Return the same error if it is not.
    get_path_as_uri_or_error(path).map(String::from)
}

/// Converts a path to a URI and returns it as a string.
/// The path must be absolute.
///
/// This is a preferred way if you are optimistic and want unrecoverable error on failure.
///
/// # Panics
///
/// Panics if the path is not absolute. Use [`get_path_as_uri_string_or_error`] to handle that case.
///
/// # Examples
///
/// ```rust
/// # #[cfg(unix)]
/// # {
/// use std::path::{Path, PathBuf};
/// use path_as_uri::get_path_as_uri_string;
///
/// assert_eq!(get_path_as_uri_string(Path::new("/path/to/file.txt")), "file:///path/to/file.txt");
/// # }
/// ```
#[must_use]
pub fn get_path_as_uri_string(path: &PathThatMustBeAbsolute) -> PathAsUriAsString {
    #[expect(
        clippy::expect_used,
        reason = "the documented panicking variant; the fallible one is get_path_as_uri_string_or_error"
    )]
    get_path_as_uri_string_or_error(path).expect("the path must be absolute")
}

#[cfg(test)]
mod tests;
