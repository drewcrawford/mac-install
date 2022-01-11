Rust crate to install a macOS package.  Useful for sysadmin tasks.

This crate provides an asynchronous API that can be used with any async runtime.  It is tested with [kiruna](https://github.com/drewcrawford/kiruna/).

# Example

```rust
async fn example() -> Result<(), mac_install::Error> {
    mac_install::install(std::path::Path::new("testdata/test.pkg"),"mypassword".to_owned(),kiruna::Priority::Testing).await
}
```

# Similar libraries

Since Rust is a compiled language, its binaries are self-contained.  Therefore you can write tools to bring up a production or
development environment in Rust itself, compile them, and shoot them over to new servers via SSH.

You might be interested my expanded universe of sysadmin libraries:

* [rustupr](https://github.com/drewcrawford/rustupr), which installs Rust
* [github-actions-runner](https://github.com/drewcrawford/github-actions-runner), which installs GitHub's action runner
* [dmg](https://github.com/drewcrawford/dmg) to mount DMG images