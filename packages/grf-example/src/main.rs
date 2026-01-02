use grf::{GRF_HEADER_SIZE, archive::Archive};

fn main() {
    println!("Opening GRF file...");
    let mut reader = std::fs::File::open("./data.grf").unwrap();
    let grf = Archive::from_reader(&mut reader).unwrap();
    println!("GRF file opened!");
    println!("Size of header: {}", GRF_HEADER_SIZE);
    println!("Header: {:?}", grf.header);
    println!("File count: {:?}", grf.file_table.files.len());

    for file in grf.file_table.files {
        println!("File: {:?}", file.filename);
    }
}
