/*!
Rust crate to install a macOS package.  Useful for sysadmin tasks.

This crate provides an asynchronous API that can be used with any async runtime.  It is tested with [kiruna](https://github.com/drewcrawford/kiruna/).

# Example

```
async fn example() -> Result<(), mac_install::Error> {
    mac_install::install(std::path::Path::new("testdata/test.pkg"),"mypassword".to_owned(),kiruna::Priority::Testing).await
}
```
*/
use std::path::Path;
use command_rs::Sudo;
use std::fmt::Formatter;
use kiruna::Priority;

#[derive(Debug)]
pub enum Error {
    InstallerReturnCode(i32),
    ExecutionError(command_rs::Error)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}",self))
    }
}
impl From<command_rs::Error> for Error {
    fn from(f: command_rs::Error) -> Self {
        Self::ExecutionError(f)
    }
}
impl std::error::Error for Error {}

/**Installs a macOS package.

# Example
```
async fn example() -> Result<(), mac_install::Error> {
    mac_install::install(std::path::Path::new("testdata/test.pkg"),"mypassword".to_owned(),kiruna::Priority::Testing).await
}
```
*/
pub async fn install(path: &Path,password: String, priority: Priority) -> Result<(), Error> {
    let mut task = Sudo::new("installer", password);

        task.arg("-pkg")
        .arg(path)
        .arg("-target")
        .arg("/");
    let result =         task.status(priority).await?;

    if result.code().unwrap() == 0 {
        Ok(())
    }
    else {
        Err(Error::InstallerReturnCode(result.code().unwrap()))
    }

}

#[test] fn install_test() {
    use kiruna::test::test_await;
    let f = install(Path::new("testdata/test.pkg"),"".to_owned(),Priority::Testing);
    let result = test_await(f, std::time::Duration::from_secs(5));
    match result.err() {
        None => {}
        Some(Error::InstallerReturnCode(1)) => {
            //likely a sudo error
        }
        Some(error) => {
            panic!("{}",error);
        }
    }
}