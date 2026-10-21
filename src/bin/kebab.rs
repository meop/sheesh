use sheesh::data::Trailer;
#[cfg(windows)]
use sheesh::windows_pe;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("--version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if let Err(e) = run() {
        eprintln!("kebab: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (source_path, target_path, stamped_args) = parse_args()?;

    // the path as the shim will find it, for its subsystem; the trailer keeps it as written
    let resolved = PathBuf::from(sheesh::expand_env(&source_path.to_string_lossy()));
    let template = select_template(&resolved)?;
    fs::copy(&template, &target_path)?;

    let mut dest = fs::OpenOptions::new().append(true).open(&target_path)?;
    Trailer::append_to(&source_path, &stamped_args, &mut dest)?;

    Ok(())
}

fn parse_args() -> Result<(PathBuf, PathBuf, Vec<String>), Box<dyn Error>> {
    let mut source: Option<PathBuf> = None;
    let mut target: Option<PathBuf> = None;
    let mut stamped_args = Vec::new();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source-path" => {
                source = Some(PathBuf::from(
                    args.next().ok_or("--source-path requires a value")?,
                ));
            }
            "--target-path" => {
                target = Some(PathBuf::from(
                    args.next().ok_or("--target-path requires a value")?,
                ));
            }
            "--arg" => {
                stamped_args.push(args.next().ok_or("--arg requires a value")?);
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    let source = source.ok_or("--source-path is required")?;
    let target = target.ok_or("--target-path is required")?;
    Ok((source, target, stamped_args))
}

fn select_template(source_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let shim_dir = env::current_exe()?
        .parent()
        .ok_or("cannot locate kebab exe dir")?
        .to_path_buf();

    #[cfg(windows)]
    {
        let use_gui = matches!(windows_pe::detect(source_path)?, windows_pe::Subsystem::Gui);
        let name = if use_gui {
            "sheesh-gui.exe"
        } else {
            "sheesh.exe"
        };
        Ok(shim_dir.join(name))
    }

    #[cfg(unix)]
    {
        let _ = source_path;
        Ok(shim_dir.join("sheesh"))
    }
}
