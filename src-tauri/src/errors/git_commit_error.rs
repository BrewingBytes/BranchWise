#[derive(Debug, PartialEq)]
pub enum GitCommitError {
    DecompressionError,
    InvalidCommitFile,
}
