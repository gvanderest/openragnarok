use std::io::{Read, Seek, SeekFrom};

use binrw::{BinRead, binrw};

use crate::error;

#[binrw]
#[br(little)]
#[derive(Debug)]
pub struct CompressedFileTable {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    // pub files: Vec<GrfCompressedFile>,
}

impl CompressedFileTable {
    pub fn from_reader_with_offset<R: Read + Seek>(
        reader: &mut R,
        offset: u32,
    ) -> Result<Self, error::Error> {
        // FIXME: Replace unwrap
        reader.seek(SeekFrom::Start(offset as u64)).unwrap();
        let table = Self::read(reader);
        match table {
            Ok(table) => {
                // Everything is good, valid table
                return Ok(table);
            }
            Err(err) => {
                panic!("Unexpected error: {}", err)
            }
        }
    }
}
