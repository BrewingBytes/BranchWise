#[derive(Debug, PartialEq)]
pub enum GitObjectError {
    DecompressionError,
    InvalidCommitFile,
    FileReadError,
}
