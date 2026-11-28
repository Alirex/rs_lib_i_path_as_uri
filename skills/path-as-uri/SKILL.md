---
name: path-as-uri
description: Convert an absolute filesystem path to a file:// URI in Rust with the path_as_uri crate. Use when a Rust program prints or logs file paths that should be clickable, or when the user asks for a file URI from a Path.
---

# path_as_uri

Add the crate with `cargo add path_as_uri`, then pick one function by how the caller treats failure:

| Function                                 | Returns                            | On a relative path |
| ---------------------------------------- | ---------------------------------- | ------------------ |
| `get_path_as_uri_or_error(&Path)`        | `Result<url::Url, PathAsUriError>` | error              |
| `get_path_as_uri_string_or_error(&Path)` | `Result<String, PathAsUriError>`   | error              |
| `get_path_as_uri_string(&Path)`          | `String`                           | panics             |

## Steps

1. Make the path absolute first (`std::path::absolute` or `canonicalize`); a relative path is never converted.
2. In library code or anywhere input is not trusted, use an `*_or_error` function and handle the error.
3. Use the short `get_path_as_uri_string` only where the path is known to be absolute.
4. Do not build `file://` strings by hand: spaces and special characters need encoding, which the crate does.

## Rules

- This skill runs no scripts and fetches nothing; it tells you which command or API to use.
- Read the README before you change how the project is used; the repo's own rules for agents are in AGENTS.md.
