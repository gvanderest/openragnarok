use std::io::{Read, Seek};

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
        let file_table =
            file_table::CompressedFileTable::from_reader_with_offset(reader, header.offset)?;
        Ok(Archive {
            reader: Box::new(reader),
            header,
            file_table,
        })
    }
}
