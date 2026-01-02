pub struct CompressedFileEntry {
    pub raw_filename: Vec<char>,
    pub filename: String,
    pub compressed_size: u32,
    pub compressed_size_aligned: u32,
    pub size: u32,
    pub flags: u8,
    pub offset: u32,
}
