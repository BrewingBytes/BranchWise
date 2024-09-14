#[derive(Debug, PartialEq)]
pub enum GitObjectError {
    DecompressionError,
    InvalidObjectFile(ObjectError),
    InvalidCommitFile,
    InvalidBlobFile,
    InvalidTreeFile,
    FileReadError,
    ParsingError,
    ShaError,
}

#[derive(Debug, PartialEq)]
pub enum ObjectError {
    InvalidHeader,
    InvalidContent,
}
