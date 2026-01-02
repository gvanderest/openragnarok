#[derive(Debug)]
pub enum Error {
    EncryptionUnsupported,
    InvalidSeek(u32),
    InvalidSignature,
    VersionUnsupported(u32),
}
