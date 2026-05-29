#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use std::ffi::OsString;
use std::path::Path;

pub fn spawn_target(target: &Path, args: &[OsString]) -> Result<u32, Box<dyn std::error::Error>> {
    #[cfg(windows)]
    return windows::spawn_target(target, args);
    #[cfg(unix)]
    return unix::spawn_target(target, args);
    #[allow(unreachable_code)]
    Err("unsupported platform".into())
}
