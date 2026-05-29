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
    if !t.source_path.exists() {
        return Err(format!("source not found: {}", t.source_path.display()).into());
    }
    let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    platform::spawn_target(&t.source_path, &args)
}
