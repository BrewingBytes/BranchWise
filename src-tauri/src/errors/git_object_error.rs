#[derive(Debug, PartialEq)]
pub enum GitObjectError {
    DecompressionError,
    InvalidCommitFile,
    InvalidBlobFile,
    FileReadError,
    ParsingError,
    ShaError,
}
