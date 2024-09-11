use std::fmt;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum GitFilesRequired {
    HEAD,
    CONFIG,
}

impl fmt::Display for GitFilesRequired {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitFilesRequired::HEAD => write!(f, "HEAD"),
            GitFilesRequired::CONFIG => write!(f, "config"),
        }
    }
}

pub enum GitFilesOptional {
    PackedRefs,
}

impl fmt::Display for GitFilesOptional {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitFilesOptional::PackedRefs => write!(f, "packed-refs"),
        }
    }
}
