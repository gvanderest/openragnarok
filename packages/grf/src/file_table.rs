use std::io::{Read, Seek, SeekFrom};

use binrw::{BinRead, binrw};

use crate::{error, file_entry};

#[binrw]
#[br(little, import { file_count: u32 })]
#[derive(Debug)]
pub struct CompressedFileTable {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    #[br(count = usize::try_from(file_count).unwrap())]
    pub files: Vec<file_entry::CompressedFileEntry>,
}

impl CompressedFileTable {
    pub fn from_reader_with_offset_and_file_count<R: Read + Seek>(
        reader: &mut R,
        offset: u32,
        file_count: u32,
    ) -> Result<Self, error::Error> {
        if reader.seek(SeekFrom::Start(offset as u64)).is_err() {
            return Err(error::Error::InvalidSeek(offset));
        }

        let table = Self::read_args(reader, CompressedFileTableBinReadArgs { file_count });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_happy_path_with_no_files() {
        let mut reader = std::io::Cursor::new(
            b"\
            \x00\x00\x00\x00\
            \x01\x00\x00\x00\
            \x05\x00\x00\x00\
            ",
        );
        let result =
            CompressedFileTable::from_reader_with_offset_and_file_count(&mut reader, 4, 0).unwrap();
        assert!(result.compressed_size == 1);
        assert!(result.uncompressed_size == 5);
        assert!(result.files.len() == 0);
    }

    #[test]
    fn test_basic_happy_path_with_one_file() {
        let mut reader = std::io::Cursor::new(
            b"\
            \x00\x00\x00\x00\
            \x01\x00\x00\x00\
            \x05\x00\x00\x00\
            \
            Example File\0\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\x00\x00\x00\
            \x00\
            \x00\x00\x00\x00\
            ",
        );
        let result =
            CompressedFileTable::from_reader_with_offset_and_file_count(&mut reader, 4, 1).unwrap();
        assert!(result.compressed_size == 1);
        assert!(result.uncompressed_size == 5);
        assert!(result.files.len() == 1);
        assert!(result.files.get(0).unwrap().filename.to_string() == "Example File");
    }
}
