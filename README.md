# Path as URI

---

[![pre-commit](https://github.com/Alirex/rs_lib_i_path_as_uri/actions/workflows/pre-commit.yaml/badge.svg)](https://github.com/Alirex/rs_lib_i_path_as_uri/actions/workflows/pre-commit.yaml)

Allows you to get the path of a file as a URI. That can be clicked in the terminal.

## Example

---

```rust
use path_as_uri::get_path_as_uri_string;

fn main() {
    let path = std::path::PathBuf::new("/path/to/file.txt");
    let uri = get_path_as_uri_string(&path);
    println!("URI: {}", uri);
}
```

## 🤔 Why?

### Why do I need this?

In some apps, I want to provide a clickable path to a file in the terminal from the app.

Uri is a working solution for this.

In Python, you can make a similar solution using `pathlib` module:

- https://docs.python.org/3/library/pathlib.html#pathlib.Path.as_uri

### Why do I need this library?

I'm tired of some repeatable boilerplate code.
I just want to call the function and get the result.
In different small apps.

So I created this library.

For now, it is a relatively simple wrapper around [url](https://crates.io/crates/url) library.

### Why just don't concatenate the string?

Yes, you can try to concatenate your path with `file://` prefix.

But it will not work for all cases.

For example, if you have a path with spaces in it, you need to encode it.

So, for me, it is easier to use a library that does it for me.

## 🛠️ Dev

### Register pre-commit hooks

```shell
pre-commit install
```

### Run pre-commit hooks

```shell
pre-commit run --all-files
```

### Upgrade dependencies

```shell
cargo upgrade
```

- [cargo upgrade](https://crates.io/crates/cargo-edit#cargo-upgrade)
  - `cargo install cargo-edit --no-default-features --features "upgrade"`

## Other

Maybe you will be interested in:

- Ways to make a path absolute:
  - https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    - Strict canonicalization with interaction with the filesystem.
  - https://crates.io/crates/path-absolutize
    - Does not care about whether the file exists and what the file really is.
  - https://doc.rust-lang.org/beta/std/path/fn.absolute.html
    - May be interesting instead of `path-absolutize` when it will out of beta.
