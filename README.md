Rust crate to install a macOS package.  Useful for sysadmin tasks.

This crate provides an asynchronous API that can be used with any async runtime.  It is tested with [kiruna](https://github.com/drewcrawford/kiruna/).

# Example

```rust
async fn example() -> Result<(), mac_install::Error> {
    mac_install::install(std::path::Path::new("testdata/test.pkg"),"mypassword".to_owned(),kiruna::Priority::Testing).await
}
```