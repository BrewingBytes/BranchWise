#[derive(Debug)]
pub enum GitCommitError {
    DecompressionError,
    InvalidCommitFile,
}
