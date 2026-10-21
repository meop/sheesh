pub mod data;
mod platform;
pub mod windows_pe;

pub fn run() -> ! {
    match try_run() {
        Ok(code) => std::process::exit(code as i32),
        Err(e) => {
            eprintln!("shim: {e}");
            std::process::exit(1);
        }
    }
}

fn try_run() -> Result<u32, Box<dyn std::error::Error>> {
    let t =
        data::Trailer::read_from_self().ok_or("no shim data — use kebab to stamp a source path")?;
    let source_path = std::path::PathBuf::from(expand_env(&t.source_path.to_string_lossy()));
    if !source_path.exists() {
        return Err(format!("source not found: {}", source_path.display()).into());
    }
    // the stamped arguments first, then the shim's own
    let args: Vec<std::ffi::OsString> = t
        .args
        .iter()
        .map(|a| expand_env(a).into())
        .chain(std::env::args_os().skip(1))
        .collect();
    platform::spawn_target(&source_path, &args)
}

/// on windows a stamped `%NAME%` is the environment's NAME when the shim runs, so one shim serves every user (a
/// machine-wide shell set to `%USERPROFILE%\...`); a unix path is taken as written
pub fn expand_env(s: &str) -> String {
    if cfg!(windows) {
        data::expand(s, |k| std::env::var(k).ok())
    } else {
        s.to_string()
    }
}
