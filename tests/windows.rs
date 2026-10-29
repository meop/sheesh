#![cfg(windows)]

use std::path::PathBuf;
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wedge-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_shim_stamped_through_a_pointer_runs_what_the_pointer_names_now() {
    let dir = scratch("pointer");
    let system = PathBuf::from(std::env::var("SystemRoot").unwrap()).join("System32");
    for (version, program) in [("1", "cmd.exe"), ("2", "hostname.exe")] {
        std::fs::create_dir_all(dir.join(version)).unwrap();
        std::fs::copy(system.join(program), dir.join(version).join("tool.exe")).unwrap();
    }
    let latest = dir.join("latest");
    std::fs::write(&latest, dir.join("1").display().to_string()).unwrap();
    let shim = dir.join("shimmed.exe");
    assert!(
        Command::new(env!("CARGO_BIN_EXE_mallet"))
            .arg("--source-path")
            .arg(latest.join("tool.exe"))
            .arg("--target-path")
            .arg(&shim)
            .status()
            .unwrap()
            .success()
    );
    let stamped = std::fs::read(&shim).unwrap();

    let out = Command::new(&shim)
        .args(["/c", "echo", "one"])
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "one");
    std::fs::write(&latest, dir.join("2").display().to_string()).unwrap();
    let out = Command::new(&shim).output().unwrap();
    assert!(
        String::from_utf8_lossy(&out.stdout)
            .trim()
            .eq_ignore_ascii_case(&std::env::var("COMPUTERNAME").unwrap()),
        "the shim did not follow the pointer"
    );
    assert_eq!(std::fs::read(&shim).unwrap(), stamped, "the shim changed");
}
