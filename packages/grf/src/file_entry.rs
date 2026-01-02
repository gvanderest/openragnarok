use binrw::{NullString, binrw};

#[binrw]
#[br(little)]
#[derive(Debug)]
pub struct CompressedFileEntry {
    pub filename: NullString,
    pub compressed_size: u32,
    pub compressed_size_aligned: u32,
    pub size: u32,
    pub flags: u8,
    pub offset: u32,
}
