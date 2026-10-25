# Path as URI

[![CI](https://github.com/Alirex/rs_lib_i_path_as_uri/actions/workflows/ci.yml/badge.svg)](https://github.com/Alirex/rs_lib_i_path_as_uri/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/path_as_uri.svg)](https://crates.io/crates/path_as_uri)
[![docs.rs](https://img.shields.io/docsrs/path_as_uri)](https://docs.rs/path_as_uri)
[![license: ISC](https://img.shields.io/crates/l/path_as_uri.svg)](LICENSE.txt)

Allows you to get the path of a file as a URI. That can be clicked in the terminal.

## Install

```shell
cargo add path_as_uri
```

## Example

```rust
use path_as_uri::get_path_as_uri_string;
use std::path::Path;

fn main() {
    // The path must be absolute.
    let path = Path::new("/path/to/file with spaces.txt");
    let uri = get_path_as_uri_string(path);
    println!("URI: {uri}"); // URI: file:///path/to/file%20with%20spaces.txt
}
```

Three functions, from the most careful to the most optimistic:

- `get_path_as_uri_or_error` returns a `url::Url` or an error.
- `get_path_as_uri_string_or_error` returns a `String` or an error.
- `get_path_as_uri_string` returns a `String` and panics on a relative path.

The minimum supported Rust version is in `Cargo.toml` (`rust-version`) and is checked in CI.

## 🤔 Why?

### Why do I need this?

In some apps, I want to provide a clickable path to a file in the terminal from the app.

Uri is a working solution for this.

In Python, you can make a similar solution using `pathlib` module:

- <https://docs.python.org/3/library/pathlib.html#pathlib.Path.as_uri>

### Why do I need this library?

I'm tired of some repeatable boilerplate code.
I just want to call the function and get the result.
In different small apps.

So I created this library.

For now, it is a relatively simple wrapper around the [url](https://crates.io/crates/url) library.

### Why just don't concatenate the string?

Yes, you can try to concatenate your path with `file://` prefix.

But it will not work for all cases.

For example, if you have a path with spaces in it, you need to encode it.

So, for me, it is easier to use a library that does it for me.

## 🛠️ Dev

Tools: [just](https://just.systems), [prek](https://github.com/j178/prek), Rust via rustup (the toolchain is pinned in
`rust-toolchain.toml`), [cargo-deny](https://github.com/EmbarkStudios/cargo-deny).

```shell
just dev-hooks   # install the git hook (once per clone)
just q-check     # the full gate: hooks (fmt, clippy, tests, docs), cargo deny, cargo package
just             # all recipes
```

Release: [CHANGELOG.md](CHANGELOG.md), then a `vX.Y.Z` tag runs `.github/workflows/release.yml`, which publishes to
crates.io by trusted publishing. Rules for contributors and agents: [AGENTS.md](AGENTS.md).
