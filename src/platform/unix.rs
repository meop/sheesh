use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub fn spawn_target(target: &Path, args: &[OsString]) -> Result<u32, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(target);
    // the target runs under the shim's name, as through a symlink, so whatever reads its argv (ps, a terminal
    // multiplexer saving the session) sees `claude` rather than the versioned path behind it
    if let Some(arg0) = std::env::args_os().next() {
        cmd.arg0(arg0);
    }
    Err(cmd.args(args).exec().into())
}
