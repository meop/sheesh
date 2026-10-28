use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub fn spawn_target(
    target: &Path,
    stamped: &[OsString],
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(target);
    // the target runs under the shim's name, as through a symlink, so whatever reads its argv (ps, a terminal
    // multiplexer saving the session) sees `claude` rather than the versioned path behind it
    let mut own = std::env::args_os();
    if let Some(arg0) = own.next() {
        cmd.arg0(arg0);
    }
    Err(cmd.args(stamped).args(own).exec().into())
}

pub fn alert(message: &str) {
    eprintln!("{message}");
}
