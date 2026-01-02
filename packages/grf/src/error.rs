#[derive(Debug)]
pub enum Error {
    EncryptionUnsupported,
    InvalidSignature,
    VersionUnsupported(u32),
}
