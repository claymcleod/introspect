<p align="center">
  <h1 align="center">
    introspect
  </h1>

  <p align="center">
    <a href="https://github.com/claymcleod/introspect/actions/workflows/ci.yml" target="_blank">
      <img alt="CI: Status" src="https://github.com/claymcleod/introspect/actions/workflows/ci.yml/badge.svg" />
    </a>
    <!-- <a href="https://crates.io/crates/introspect" target="_blank">
      <img alt="crates.io version" src="https://img.shields.io/crates/v/introspect">
    </a> -->
    <!-- <img alt="crates.io downloads" src="https://img.shields.io/crates/d/introspect"> -->
    <a href="https://github.com/claymcleod/introspect/blob/master/LICENSE-APACHE" target="_blank">
      <img alt="License: Apache 2.0" src="https://img.shields.io/badge/license-Apache 2.0-blue.svg" />
    </a>
    <a href="https://github.com/claymcleod/introspect/blob/master/LICENSE-MIT" target="_blank">
      <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-blue.svg" />
    </a>
  </p>


  <p align="center">
    A crate for performing introspection on Rust structs and enums.
    <!-- <br />
    <a href="https://docs.rs/introspect"><strong>Explore the docs »</strong></a> -->
    <br />
    <br />
    <a href="https://github.com/claymcleod/introspect/issues/new?assignees=&title=Descriptive%20Title&labels=enhancement">Request Feature</a>
    ·
    <a href="https://github.com/claymcleod/introspect/issues/new?assignees=&title=Descriptive%20Title&labels=bug">Report Bug</a>
    ·
    ⭐ Consider starring the repo! ⭐
    <br />
    <br />
  </p>
</p>

_Note: currently, only the identifier name and the optional documentation for each
supported entity and member are implemented. That was all that was needed at the time
the crate was developed. However, we will implement requests and/or accept pull requests
that add additional introspection—please just leave an issue on the [issues
page](https://github.com/claymcleod/introspect/issues)!_

## 📚 Getting Started

You can add `introspect` as a dependency via the Github repository. 

```bash
cargo add --git https://github.com/claymcleod/introspect.git introspect
```

You can then use the `introspect::Introspected` trait and related traits to pull out the
information you wish. Typically, you will do with the `introspect::Introspect` derive
macro like so.

```rust
use introspect::Entity;
use introspect::Introspect;
use introspect::IntrospectedEntity;
use introspect::IntrospectedMembers;
use introspect::Member;

/// This is the documentation for the [`Example`] enum.
///
/// We can add more text down here.
#[allow(dead_code)]
#[derive(Introspect)]
enum Example {
    /// The first variant.
    One,

    /// The second variant.
    ///
    /// And some more text.
    Two,
}

fn main() {
    // Access to the top-level entity characteristics.
    match Example::introspected_entity() {
        Entity::Enum(entity) => {
            dbg!(entity.identifier());
            dbg!(entity.documentation());
        }
        _ => unreachable!(),
    }

    // Access to the members of the entity.
    for member in Example::introspected_members() {
        match member {
            Member::Variant(member) => {
                dbg!(member.identifier());
                dbg!(member.documentation());
            }
            _ => unreachable!(),
        }
    }
}
```

## Examples

You can also take a look at the
[examples](https://github.com/claymcleod/introspect/tree/main/introspect/examples) to
get a sense of the various ways you can use the crate.


## 🖥️ Development

To bootstrap a development environment, please use the following commands.

```bash
# Clone the repository
git clone git@github.com:claymcleod/introspect.git
cd introspect 

# Build the crate in release mode
cargo build --release

# List out the examples
cargo run --release --example
```

## 🚧️ Tests

Before submitting any pull requests, please make sure the code passes the following
checks.

```bash
# Run the project's tests.
cargo test --all-features

# Ensure the project doesn't have any linting warnings.
cargo clippy --all-features

# Ensure the project passes `cargo fmt`.
cargo fmt --check

# Ensure the docs build successfully.
cargo doc
```

## Minumum Supported Rust Version (MSRV)

This crate is designed to work with Rust version 1.74.0 or later. It may, by
happenstance, work with earlier versions of Rust.

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check [issues
page](https://github.com/claymcleod/introspect/issues).

## 📝 License

This project is licensed as either [Apache 2.0][license-apache] or [MIT][license-mit] at
your discretion.

Copyright © 2023-Present [Clay McLeod](https://github.com/claymcleod).

[license-apache]: https://github.com/claymcleod/introspect/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/claymcleod/introspect/blob/master/LICENSE-MIT
