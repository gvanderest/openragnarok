use grf::{GRF_HEADER_SIZE, Grf};

fn main() {
    println!("Opening GRF file...");
    let mut reader = std::fs::File::open("./data.grf").unwrap();
    let grf = Grf::from_reader(&mut reader).unwrap();
    println!("GRF file opened!");
    println!("Size of header: {}", GRF_HEADER_SIZE);
    println!("Header: {:?}", grf.header);
}
