use binrw::{BinRead, binrw};
use std::io::{Read, Seek};

use crate::error;

static FILE_COUNT_ADJUSTMENT: u32 = 7;

// Reference: https://z0q.neocities.org/ragnarok-online-formats/grf/
#[binrw]
#[br(little, magic = b"Master of Magic\0")]
#[derive(Debug)]
pub struct Header {
    pub encryption: [u8; 14],
    pub offset: u32,
    pub seed: u32,
    pub file_count: u32,
    pub version: u32,
}

impl Header {
    pub fn from_reader<T>(reader: &mut T) -> Result<Header, error::Error>
    where
        T: Read + Seek,
    {
        let header = Header::read(reader);
        match header {
            Ok(mut header) => {
                // Validate encryption
                if header.encryption != [0; 14] {
                    return Err(error::Error::EncryptionUnsupported);
                }

                // Validate version
                match header.version {
                    0x200 => {
                        header.file_count -= FILE_COUNT_ADJUSTMENT;
                    }
                    0x102 | 0x103 => {
                        header.file_count = header.file_count - header.seed - FILE_COUNT_ADJUSTMENT;
                    }
                    _ => {
                        return Err(error::Error::VersionUnsupported(header.version));
                    }
                }

                // Everything is good, valid header
                return Ok(header);
            }
            Err(err) => {
                // Handle invalid signature
                if err.to_string().contains("bad magic") {
                    return Err(error::Error::InvalidSignature);
                }

                panic!("Unexpected error: {}", err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn handle_invalid_signature() {
        let mut reader = Cursor::new(
            b"\
            Invalid String\0\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x07\x00\x00\x00\
            \x00\x02\x00\x00\
            ",
        );
        let result = Header::from_reader(&mut reader);
        assert!(matches!(result, Err(error::Error::InvalidSignature)));
    }

    #[test]
    fn handle_unsupported_encryption() {
        let mut reader = Cursor::new(
            b"\
            Master of Magic\0\
            \x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x07\x00\x00\x00\
            \x00\x01\x00\x00\
            ",
        );
        let result = Header::from_reader(&mut reader);
        assert!(matches!(result, Err(error::Error::EncryptionUnsupported)));
    }

    #[test]
    fn handle_unsupported_version() {
        let mut reader = Cursor::new(
            b"\
            Master of Magic\0\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x07\x00\x00\x00\
            \x00\x00\x00\x00\
            ",
        );
        let result = Header::from_reader(&mut reader);
        assert!(matches!(
            result,
            Err(error::Error::VersionUnsupported(0x000))
        ));
    }

    #[test]
    fn handle_valid_grf() {
        let mut reader = Cursor::new(
            b"\
            Master of Magic\0\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x07\x00\x00\x00\
            \x00\x02\x00\x00\
            ",
        );
        let result = Header::from_reader(&mut reader).unwrap();
        assert!(result.version == 0x200);
    }
}
