#[cfg(windows)]
mod windows;
#[cfg(unix)]
mod unix;

use std::path::Path;

pub fn spawn_target(target: &Path, args: &[String]) -> Result<u32, Box<dyn std::error::Error>> {
    #[cfg(windows)]
    return windows::spawn_target(target, args);
    #[cfg(unix)]
    return unix::spawn_target(target, args);
    #[allow(unreachable_code)]
    Err("unsupported platform".into())
}
