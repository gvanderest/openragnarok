use std::io::SeekFrom;
use std::io::{Read, Seek};

use crate::GRF_HEADER_SIZE;
use crate::error;
use crate::file_table;
use crate::header;

pub struct Archive<'a> {
    reader: Box<dyn Read + 'a>,
    pub header: header::Header,
    pub file_table: file_table::CompressedFileTable,
}

impl<'a> Archive<'a> {
    pub fn from_reader<T>(reader: &'a mut T) -> Result<Self, error::Error>
    where
        T: Read + Seek,
    {
        // Read in header
        let header = header::Header::from_reader(reader)?;

        // Move to compressed file table, assume version 0x200 only for now
        // FIXME: Handle 0x1XX versions too
        reader
            .seek(SeekFrom::Start(
                GRF_HEADER_SIZE as u64 + header.offset as u64 + 8,
            ))
            .unwrap();
        let file_table = file_table::CompressedFileTable::from_reader_with_file_count(
            reader,
            header.file_count,
        )?;

        Ok(Archive {
            reader: Box::new(reader),
            header,
            file_table,
        })
    }
}
