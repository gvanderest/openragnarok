use std::io::{Read, Seek};

use binrw::{BinRead, binrw};

pub static GRF_HEADER_SIZE: usize = 46;

// Reference: https://z0q.neocities.org/ragnarok-online-formats/grf/
#[binrw]
#[br(little, magic = b"Master of Magic\0")]
#[derive(Debug)]
pub struct GrfHeader {
    pub encryption: [u8; 14],
    pub offset: u32,
    pub seed: u32,
    pub file_count: u32,
    pub version: u32,
}

impl GrfHeader {
    pub fn from_reader<T>(reader: &mut T) -> Result<GrfHeader, GrfError>
    where
        T: Read + Seek,
    {
        let header = GrfHeader::read(reader);
        match header {
            Ok(header) => {
                // Validate encryption
                if header.encryption != [0; 14] {
                    return Err(GrfError::EncryptionUnsupported);
                }

                // Validate version
                if header.version != 0x200 {
                    return Err(GrfError::VersionUnsupported(header.version));
                }

                // Everything is good, valid header
                return Ok(header);
            }
            Err(err) => {
                // Handle invalid signature
                if err.to_string().contains("bad magic") {
                    return Err(GrfError::InvalidSignature);
                }

                panic!("Unexpected error: {}", err)
            }
        }
    }
}

// FIXME: Handle versions 0x100 - 0x103 uncompressed file tables

// Used in Version 0x200
pub struct GrfCompressedFileTable {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub files: Vec<GrfCompressedFile>,
}

pub struct GrfCompressedFile {
    pub raw_filename: Vec<char>,
    pub filename: String,
    pub compressed_size: u32,
    pub compressed_size_aligned: u32,
    pub size: u32,
    pub flags: u8,
    pub offset: u32,
}

pub struct Grf<'a> {
    reader: Box<dyn Read + 'a>,
    pub header: GrfHeader,
    // pub file_table: GrfCompressedFileTable,
}

#[derive(Debug)]
pub enum GrfError {
    EncryptionUnsupported,
    InvalidSignature,
    VersionUnsupported(u32),
}

impl<'a> Grf<'a> {
    pub fn from_reader<T>(reader: &'a mut T) -> Result<Self, GrfError>
    where
        T: Read + Seek,
    {
        let header = GrfHeader::from_reader(reader)?;
        Ok(Grf {
            reader: Box::new(reader),
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn handle_invalid_signature() {
        let mut reader = Cursor::new(
            b"\
            Invalid String\0\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x02\x00\x00\
            ",
        );
        let result = Grf::from_reader(&mut reader);
        assert!(matches!(result, Err(GrfError::InvalidSignature)));
    }

    #[test]
    fn handle_unsupported_encryption() {
        let mut reader = Cursor::new(
            b"\
            Master of Magic\0\
            \x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x01\x00\x00\
            ",
        );
        let result = Grf::from_reader(&mut reader);
        assert!(matches!(result, Err(GrfError::EncryptionUnsupported)));
    }

    #[test]
    fn handle_unsupported_version() {
        let mut reader = Cursor::new(
            b"\
            Master of Magic\0\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            ",
        );
        let result = Grf::from_reader(&mut reader);
        assert!(matches!(result, Err(GrfError::VersionUnsupported(0x000))));
    }

    #[test]
    fn handle_valid_grf() {
        let mut reader = Cursor::new(
            b"\
            Master of Magic\0\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x02\x00\x00\
            ",
        );
        let result = Grf::from_reader(&mut reader).unwrap();
        assert!(result.header.version == 0x200);
    }
}
