use std::fmt;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum GitFolders {
    REFS,
    OBJECTS,
    HOOKS
}

impl fmt::Display for GitFolders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitFolders::REFS => write!(f, "refs"),
            GitFolders::OBJECTS => write!(f, "objects"),
            GitFolders::HOOKS => write!(f, "hooks"),
        }
    }
}
