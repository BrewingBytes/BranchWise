use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;

pub const GIT_FOLDER: &str = ".git";

#[derive(EnumIter)]
pub enum GitFolders {
    REFS,
    OBJECTS,
    HOOKS,
}

impl AsRef<str> for GitFolders {
    fn as_ref(&self) -> &str {
        match *self {
            GitFolders::REFS => "refs",
            GitFolders::OBJECTS => "objects",
            GitFolders::HOOKS => "hooks",
        }
    }
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

pub enum GitObjects {
    INFO,
    PACK,
}

impl AsRef<str> for GitObjects {
    fn as_ref(&self) -> &str {
        match *self {
            GitObjects::INFO => "info",
            GitObjects::PACK => "pack",
        }
    }
}

#[derive(EnumIter, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GitRefs {
    HEADS,
    REMOTES,
    TAGS,
}

impl GitRefs {
    pub fn from(ref_str: &str) -> Option<GitRefs> {
        if ref_str == GitRefs::HEADS.as_ref() {
            return Some(GitRefs::HEADS);
        } else if ref_str == GitRefs::REMOTES.as_ref() {
            return Some(GitRefs::REMOTES);
        } else if ref_str == GitRefs::TAGS.as_ref() {
            return Some(GitRefs::TAGS);
        }

        None
    }
}

impl AsRef<str> for GitRefs {
    fn as_ref(&self) -> &str {
        match *self {
            GitRefs::HEADS => "heads",
            GitRefs::REMOTES => "remotes",
            GitRefs::TAGS => "tags",
        }
    }
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

pub enum GitBranchType {
    Local,
    Remote(String),
    Tags,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_folders_as_ref() {
        assert_eq!(GitFolders::REFS.as_ref(), "refs");
        assert_eq!(GitFolders::OBJECTS.as_ref(), "objects");
        assert_eq!(GitFolders::HOOKS.as_ref(), "hooks");
    }

    #[test]
    fn test_git_objects_as_ref() {
        assert_eq!(GitObjects::INFO.as_ref(), "info");
        assert_eq!(GitObjects::PACK.as_ref(), "pack");
    }
}
