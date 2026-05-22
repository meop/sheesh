use std::path::Path;
use std::process::Command;

pub fn spawn_target(target: &Path, args: &[String]) -> Result<u32, Box<dyn std::error::Error>> {
    let status = Command::new(target).args(args).status()?;
    Ok(status.code().unwrap_or(1) as u32)
}
