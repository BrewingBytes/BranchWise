#[derive(Debug, PartialEq)]
pub enum GitObjectError {
    CompressionError,
    DecompressionError,
    InvalidObjectFile(ObjectError),
    InvalidCommitFile(CommitError),
    InvalidBlobFile,
    InvalidTreeFile,
    FileReadError,
    ParsingError,
    ShaError,
    InvalidHash,
    PackError,
}

#[derive(Debug, PartialEq)]
pub enum ObjectError {
    InvalidHeader,
    InvalidContent,
}

#[derive(Debug, PartialEq)]
pub enum CommitError {
    InvalidHeader,
    InvalidContent,
    InvalidAuthor,
    InvalidCommiter,
}
