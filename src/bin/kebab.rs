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

    // a relative path would be found from wherever the shim is run, so it is made absolute here, from where kebab is;
    // one that opens with `%NAME%` is absolute once the shim expands it
    let source_path = if source_path.to_string_lossy().starts_with('%') {
        source_path
    } else {
        std::path::absolute(&source_path)?
    };

    // the path as the shim will find it, for its subsystem; the trailer keeps it as written
    let resolved = PathBuf::from(sheesh::expand_env(&source_path.to_string_lossy()));
    let template = select_template(&resolved)?;

    // stamped beside the shim, then moved over it: nothing ever runs a shim half written
    let name = target_path
        .file_name()
        .ok_or("--target-path has no file name")?
        .to_string_lossy()
        .into_owned();
    let staged = target_path.with_file_name(format!(".{name}.{}.new", std::process::id()));
    let stamped = (|| -> Result<(), Box<dyn Error>> {
        fs::copy(&template, &staged)?;
        let mut dest = fs::OpenOptions::new().append(true).open(&staged)?;
        Trailer::append_to(&source_path, &stamped_args, &mut dest)?;
        drop(dest);
        replace(&staged, &target_path, &name)
    })();
    if stamped.is_err() {
        let _ = fs::remove_file(&staged);
    }
    stamped
}

/// staged moved over target. On unix a rename replaces a shim that is running. On windows a running shim is open for
/// as long as its target runs, and replacing it can fail where moving it cannot, so it is moved aside first, and
/// deleted once nothing runs it: now if it can be, else by the next stamp
fn replace(staged: &Path, target: &Path, name: &str) -> Result<(), Box<dyn Error>> {
    let aside = |n: &str| n.starts_with(&format!(".{name}.")) && n.ends_with(".old");
    if let Some(dir) = target.parent().filter(|d| !d.as_os_str().is_empty()) {
        for entry in fs::read_dir(dir)?.flatten() {
            if aside(&entry.file_name().to_string_lossy()) {
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    match fs::rename(staged, target) {
        Ok(()) => return Ok(()),
        Err(e) if !target.exists() => return Err(e.into()),
        Err(_) => {}
    }
    let old = target.with_file_name(format!(".{name}.{}.old", std::process::id()));
    fs::rename(target, &old)?;
    if let Err(e) = fs::rename(staged, target) {
        let _ = fs::rename(&old, target);
        return Err(e.into());
    }
    let _ = fs::remove_file(&old);
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
