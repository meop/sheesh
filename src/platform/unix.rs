use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub fn spawn_target(target: &Path, args: &[OsString]) -> Result<u32, Box<dyn std::error::Error>> {
    Err(Command::new(target).args(args).exec().into())
}
