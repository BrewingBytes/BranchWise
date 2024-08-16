use std::fmt;
use strum_macros::EnumIter;

pub const GIT_FOLDER: &str = ".git";

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

pub enum GitRefs {
    HEADS,
    REMOTES,
    TAGS
}

impl fmt::Display for GitRefs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitRefs::HEADS => write!(f, "heads"),
            GitRefs::REMOTES => write!(f, "remotes"),
            GitRefs::TAGS => write!(f, "tags"),
        }
    }
}
