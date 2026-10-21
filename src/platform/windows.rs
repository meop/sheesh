use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn SetConsoleCtrlHandler(
        handler: Option<unsafe extern "system" fn(u32) -> i32>,
        add: i32,
    ) -> i32;
}

/// the console's Ctrl+C and Ctrl+Break reach the target too, which decides what they mean; the shim waits for it
unsafe extern "system" fn leave_to_target(_event: u32) -> i32 {
    1
}

pub fn spawn_target(target: &Path, args: &[OsString]) -> Result<u32, Box<dyn std::error::Error>> {
    // a handler routine, not SetConsoleCtrlHandler(NULL, TRUE): that would be inherited, and the target would ignore
    // Ctrl+C as well
    unsafe { SetConsoleCtrlHandler(Some(leave_to_target), 1) };
    let status = Command::new(target).args(args).status()?;
    Ok(status.code().unwrap_or(1) as u32)
}
