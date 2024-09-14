#[derive(Debug, PartialEq)]
pub enum GitObjectError {
    DecompressionError,
    InvalidCommitFile,
    InvalidBlobFile,
    InvalidTreeFile,
    FileReadError,
    ParsingError,
    ShaError,
}
