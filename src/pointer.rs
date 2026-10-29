//! A pointer file: a regular file where a stamped path has a directory, holding that directory's absolute path.

use std::path::{Component, Path, PathBuf};

/// path with every pointer file in it replaced by the directory it names
pub fn resolve(path: &Path) -> PathBuf {
    if path.exists() {
        return path.to_path_buf();
    }
    let mut resolved = PathBuf::new();
    let mut hops = 0;
    for part in path.components() {
        resolved.push(part);
        if !matches!(part, Component::Normal(_)) || !resolved.is_file() || hops == 8 {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&resolved) else {
            continue;
        };
        let dir = PathBuf::from(content.trim());
        if dir.is_absolute() {
            resolved = dir;
            hops += 1;
        }
    }
    resolved
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("wedge-pointer-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_pointer_file_stands_for_the_directory_it_names() {
        let key = scratch("one");
        fs::create_dir_all(key.join("0.74.5").join("bin")).unwrap();
        fs::write(key.join("0.74.5").join("bin").join("fzf"), "").unwrap();
        fs::write(
            key.join("latest"),
            format!("{}\n", key.join("0.74.5").display()),
        )
        .unwrap();

        assert_eq!(
            resolve(&key.join("latest").join("bin").join("fzf")),
            key.join("0.74.5").join("bin").join("fzf")
        );
        fs::create_dir_all(key.join("0.74.6").join("bin")).unwrap();
        fs::write(key.join("latest"), key.join("0.74.6").display().to_string()).unwrap();
        assert_eq!(
            resolve(&key.join("latest").join("bin").join("fzf")),
            key.join("0.74.6").join("bin").join("fzf")
        );
    }

    #[test]
    fn a_path_with_no_pointer_is_itself() {
        let key = scratch("none");
        fs::write(key.join("fzf"), "").unwrap();
        assert_eq!(resolve(&key.join("fzf")), key.join("fzf"));
        assert_eq!(
            resolve(&key.join("missing").join("fzf")),
            key.join("missing").join("fzf")
        );
    }

    #[test]
    fn a_file_that_names_no_absolute_directory_is_not_a_pointer() {
        let key = scratch("relative");
        fs::write(key.join("latest"), "0.74.5").unwrap();
        assert_eq!(
            resolve(&key.join("latest").join("fzf")),
            key.join("latest").join("fzf")
        );
    }

    #[test]
    fn pointers_that_name_each_other_stop_rather_than_loop() {
        let key = scratch("loop");
        fs::write(key.join("a"), key.join("b").display().to_string()).unwrap();
        fs::write(key.join("b"), key.join("a").display().to_string()).unwrap();
        let _ = resolve(&key.join("a").join("fzf"));
    }
}
