use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::{fs, path::Path};

const MAGIC: &[u8; 4] = b"SHIM";

pub struct Trailer {
    pub source_path: PathBuf,
}

impl Trailer {
    pub fn read_from_self() -> Option<Self> {
        let exe = std::env::current_exe().ok()?;
        let mut f = fs::File::open(&exe).ok()?;

        f.seek(SeekFrom::End(-8)).ok()?;
        let mut tail = [0u8; 8];
        f.read_exact(&mut tail).ok()?;

        if &tail[4..8] != MAGIC {
            return None;
        }
        let path_len = u32::from_le_bytes(tail[0..4].try_into().unwrap()) as i64;

        let file_len = f.seek(SeekFrom::End(0)).ok()? as i64;
        if path_len + 8 > file_len {
            return None;
        }

        f.seek(SeekFrom::End(-8 - path_len)).ok()?;
        let mut buf = vec![0u8; path_len as usize];
        f.read_exact(&mut buf).ok()?;

        let s = String::from_utf8(buf).ok()?;
        Some(Trailer {
            source_path: PathBuf::from(s),
        })
    }

    pub fn append_to(source_path: &Path, dest: &mut fs::File) -> io::Result<()> {
        use std::io::Write;
        let s = source_path.to_str().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "source path is not valid UTF-8")
        })?;
        let bytes = s.as_bytes();
        let len = bytes.len() as u32;
        dest.write_all(bytes)?;
        dest.write_all(&len.to_le_bytes())?;
        dest.write_all(MAGIC)?;
        Ok(())
    }
}
