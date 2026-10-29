pub mod data;
mod platform;
pub mod pointer;
pub mod windows_pe;

pub fn run() -> ! {
    run_reporting(|e| eprintln!("shim: {e}"))
}

pub fn run_gui() -> ! {
    run_reporting(|e| platform::alert(&format!("shim: {e}")))
}

fn run_reporting(report: impl Fn(&str)) -> ! {
    match try_run() {
        Ok(code) => std::process::exit(code as i32),
        Err(e) => {
            report(&e.to_string());
            std::process::exit(1);
        }
    }
}

fn try_run() -> Result<u32, Box<dyn std::error::Error>> {
    let t = data::Trailer::read_from_self()
        .ok_or("no shim data — use mallet to stamp a source path")?;
    let source_path = pointer::resolve(std::path::Path::new(&expand_env(
        &t.source_path.to_string_lossy(),
    )));
    if !source_path.exists() {
        return Err(format!("source not found: {}", source_path.display()).into());
    }
    let stamped: Vec<std::ffi::OsString> = t.args.iter().map(|a| expand_env(a).into()).collect();
    platform::spawn_target(&source_path, &stamped)
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
