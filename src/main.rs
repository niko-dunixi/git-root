extern crate failure;
use failure::Error;
use std::process::{Command};
use std::str;

fn main() -> Result<(), failure::Error> {
    let output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()?;
    if output.status.success() {
        print!("{}", str::from_utf8(&output.stdout)?);
    } else {
        eprint!("{}", str::from_utf8(&output.stderr)?);
    }
    Ok(())
}
