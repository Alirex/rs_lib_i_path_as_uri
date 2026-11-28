# User stories: path_as_uri

Reviewed against the code on 2026-11-28.

One entry per goal a person or agent reaches with path_as_uri, the Given/When/Then lines that confirm it, and the
test that proves it. Status: `built` (every line names a test that exists), `partial`, `planned`, `gap`.

The tests named below exist as Rust unit tests inside `src/` (`#[cfg(test)]` modules). `dossier check` finds tests only in test files and test folders, so these entries say `partial` until it reads Rust unit-test modules.

## Index

| Id   | Goal                                   | Actor          | Status  |
| ---- | -------------------------------------- | -------------- | ------- |
| US-1 | print a clickable link to a file       | Rust developer | partial |
| US-2 | handle a relative path without a crash | Rust developer | partial |

## US-1: print a clickable link to a file

```yaml
schema: 1
id: US-1
actor: Rust developer
goal: print a clickable link to a file
why: so that a reader opens the file from the terminal in one click
status: partial
accept:
  - gwt: Given an absolute path, when `get_path_as_uri_string` is called, then it returns a `file://` URI.
    test: path_as_uri_string
  - gwt: Given a path with spaces, when it is converted, then the spaces are percent-encoded.
    test: path_as_uri_string_whitespace
proof:
  cmd: cargo test
```

## US-2: handle a relative path without a crash

```yaml
schema: 1
id: US-2
actor: Rust developer
goal: handle a relative path without a crash
why: so that bad input becomes an error the app can report
status: partial
accept:
  - gwt: Given a relative path, when `get_path_as_uri_or_error` is called, then it returns an error.
    test: path_as_uri_rejects_relative
  - gwt: Given that error, when it is printed, then the message names the path.
    test: error_message_contains_the_path
proof:
  cmd: cargo test
```

## Retired

None.
