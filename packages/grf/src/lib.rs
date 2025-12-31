use std::io::Read;

pub static GRF_HEADER_SIZE: usize = 46;

// Reference: https://z0q.neocities.org/ragnarok-online-formats/grf/
#[derive(Debug)]
pub struct GrfHeader {
    pub signature: String,
    pub encryption: [u8; 14],
    pub offset: u32,
    pub seed: u32,
    pub file_count: u32,
    pub version: u32,
}

// FIXME: Handle versions 0x100 - 0x103 uncompressed file tables

// Used in Version 0x200
pub struct GrfCompressedFileTable {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub files: Vec<GrfCompressedFile>,
}

// impl GrfCompressedFileTable {
//     pub fn create_from_header(reader: &mut dyn Read, header: GrfHeader) -> Self {
//         let compressed_size = Grf::read_next_u32(reader);
//         let uncompressed_size = Grf::read_next_u32(reader);
//         let file_count = Grf::read_next_u32(reader);
//         let mut files = Vec::with_capacity(file_count as usize);

//         // for _ in 0..file_count {
//         for _ in 0..2 {
//             let raw_filename = Grf::read_next_string(reader);
//             let filename = raw_filename.clone().into_iter().collect();
//             let compressed_size = Grf::read_next_u32(reader);
//             let compressed_size_aligned = Grf::read_next_u32(reader);
//             let size = Grf::read_next_u32(reader);
//             let flags = Grf::read_next_u8(reader);
//             let offset = Grf::read_next_u32(reader);

//             files.push(GrfCompressedFile {
//                 raw_filename,
//                 filename,
//                 compressed_size,
//                 compressed_size_aligned,
//                 size,
//                 flags,
//                 offset,
//             });
//         }

//         GrfCompressedFileTable {
//             compressed_size,
//             uncompressed_size,
//             files,
//         }
//     }
// }

pub struct GrfCompressedFile {
    pub raw_filename: Vec<char>,
    pub filename: String,
    pub compressed_size: u32,
    pub compressed_size_aligned: u32,
    pub size: u32,
    pub flags: u8,
    pub offset: u32,
}

impl GrfHeader {
    fn default() -> GrfHeader {
        GrfHeader {
            signature: "".to_string(),
            encryption: [0u8; 14],
            offset: 0,
            seed: 0,
            file_count: 0,
            version: 0,
        }
    }
}

pub struct Grf<'a> {
    reader: Box<dyn Read + 'a>,
    pub header: GrfHeader,
    // pub file_table: GrfCompressedFileTable,
}

impl<'a> Grf<'a> {
    pub fn from_reader(reader: &'a mut dyn Read) -> Self {
        let header = Grf::create_header_from_reader(reader);
        // let file_table = GrfCompressedFileTable::create_from_header(reader, header);
        Grf {
            reader: Box::new(reader),
            header,
            // file_table,
        }
    }

    fn read_next_u32(reader: &mut dyn Read) -> u32 {
        let mut buffer = [0_u8; std::mem::size_of::<u32>()];
        reader.read_exact(&mut buffer).unwrap();
        u32::from_le_bytes(buffer)
    }

    fn create_header_from_reader(reader: &mut dyn Read) -> GrfHeader {
        let mut header = GrfHeader::default();

        // Magic string
        let mut raw_magic: [u8; 16] = [0; 16];
        reader.read_exact(&mut raw_magic).unwrap();

        reader.read_exact(&mut header.encryption).unwrap();

        header.offset = Grf::read_next_u32(reader);
        header.seed = Grf::read_next_u32(reader);
        header.file_count = Grf::read_next_u32(reader);
        header.version = Grf::read_next_u32(reader);

        header
    }
}
