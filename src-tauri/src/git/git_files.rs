use std::fmt;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum GitFiles {
    HEAD,
    CONFIG
}

impl fmt::Display for GitFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitFiles::HEAD => write!(f, "HEAD"),
            GitFiles::CONFIG => write!(f, "config"),
        }
    }
}
