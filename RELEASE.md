# Release

* [ ] In the workspace directory, run the following commands.

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

* [ ] For each crate, bump the version in `Cargo.toml` based on the changes that have
  been applied.
* [ ] Stage the changes: `git add .`.
* [ ] Create git commit: `git commit -m "release: bumps to version to v0.1.0"`.
* [ ] For each crate, create a git tag: `git tag introspect-core-v0.1.0`.
* [ ] Push the release and the tags: `git push && git push --tags`.
* [ ] For each crate, publish the new crate: `cargo publish --all-features`.