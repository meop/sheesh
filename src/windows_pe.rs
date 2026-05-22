use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

pub enum Subsystem {
    Gui,
    Console,
    Unknown(u16),
}

pub fn detect(path: &Path) -> io::Result<Subsystem> {
    let mut f = fs::File::open(path)?;

    let mut dos = [0u8; 64];
    f.read_exact(&mut dos)?;
    if &dos[0..2] != b"MZ" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "not a PE file"));
    }
    let e_lfanew = u32::from_le_bytes(dos[0x3C..0x40].try_into().unwrap()) as u64;

    f.seek(SeekFrom::Start(e_lfanew))?;
    let mut pe = [0u8; 94];
    f.read_exact(&mut pe)?;
    if &pe[0..4] != b"PE\0\0" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PE signature missing",
        ));
    }

    let subsystem = u16::from_le_bytes(pe[92..94].try_into().unwrap());
    Ok(match subsystem {
        2 => Subsystem::Gui,
        3 => Subsystem::Console,
        n => Subsystem::Unknown(n),
    })
}
