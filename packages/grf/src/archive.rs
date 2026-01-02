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
        let header = header::Header::from_reader(reader)?;
        let file_table = file_table::CompressedFileTable::from_reader_with_offset_and_file_count(
            reader,
            GRF_HEADER_SIZE + header.offset,
            1, // header.file_count,
        )?;
        Ok(Archive {
            reader: Box::new(reader),
            header,
            file_table,
        })
    }
}
