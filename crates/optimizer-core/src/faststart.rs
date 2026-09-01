use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub fn is_mp4_faststart<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path.as_ref())
        .with_context(|| format!("Failed to open file for faststart check: {}", path.as_ref().display()))?;

    let file_len = file.metadata()?.len();
    let mut offset: u64 = 0;
    let mut moov_offset: Option<u64> = None;
    let mut mdat_offset: Option<u64> = None;

    let mut header = [0u8; 8];

    // Read top-level atoms
    while offset + 8 <= file_len {
        file.seek(SeekFrom::Start(offset))?;
        if file.read_exact(&mut header).is_err() {
            break;
        }

        let atom_size_32 = u32::from_be_bytes([header[0], header[1], header[2], header[3]]) as u64;
        let atom_type = &header[4..8];

        let actual_size = if atom_size_32 == 1 {
            // Extended 64-bit size
            let mut ext = [0u8; 8];
            if file.read_exact(&mut ext).is_err() {
                break;
            }
            u64::from_be_bytes(ext)
        } else if atom_size_32 == 0 {
            // Atom extends to EOF
            file_len - offset
        } else {
            atom_size_32
        };

        if atom_type == b"moov" {
            moov_offset = Some(offset);
        } else if atom_type == b"mdat" && mdat_offset.is_none() {
            mdat_offset = Some(offset);
        }

        // If both found or if moov found before mdat, we can decide
        if let (Some(moov), Some(mdat)) = (moov_offset, mdat_offset) {
            return Ok(moov < mdat);
        }

        if actual_size < 8 {
            break;
        }

        offset += actual_size;
    }

    // If moov was found and mdat was not yet encountered, it's faststart
    if let Some(moov) = moov_offset {
        if let Some(mdat) = mdat_offset {
            return Ok(moov < mdat);
        }
        return Ok(true);
    }

    Ok(false)
}
