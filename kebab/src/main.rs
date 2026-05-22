#[cfg(windows)]
use sheesh::pe;
use sheesh::trailer::Trailer;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(e) = run() {
        eprintln!("kebab: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (source_path, target_path) = parse_args()?;

    let template = select_template(&source_path)?;
    fs::copy(&template, &target_path)?;

    let mut dest = fs::OpenOptions::new().append(true).open(&target_path)?;
    Trailer::append_to(&source_path, &mut dest)?;

    Ok(())
}

fn parse_args() -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
    let mut source: Option<PathBuf> = None;
    let mut target: Option<PathBuf> = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source-path" => {
                source = Some(PathBuf::from(args.next().ok_or("--source-path requires a value")?));
            }
            "--target-path" => {
                target = Some(PathBuf::from(args.next().ok_or("--target-path requires a value")?));
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    let source = source.ok_or("--source-path is required")?;
    let target = target.ok_or("--target-path is required")?;
    Ok((source, target))
}

fn select_template(source_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let shim_dir = env::current_exe()?.parent().ok_or("cannot locate kebab exe dir")?.to_path_buf();

    #[cfg(windows)]
    {
        let use_gui = matches!(pe::detect(source_path)?, pe::Subsystem::Gui);
        let name = if use_gui { "sheesh-gui.exe" } else { "sheesh.exe" };
        Ok(shim_dir.join(name))
    }

    #[cfg(unix)]
    {
        let _ = source_path;
        Ok(shim_dir.join("sheesh"))
    }
}
