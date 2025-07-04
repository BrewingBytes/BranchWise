use serde::{Deserialize, Serialize};

use crate::errors::git_object_error::GitObjectError;

use super::git_folders::{GitFolders, GitRefs};
use super::object::HASH_SIZE;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GitHead {
    Branch(GitRefs, String),
    Hash(String),
}

impl GitHead {
    /**
     * Parse the content of a HEAD file and return a GitHead object
     */
    pub fn from(content: &str) -> Result<GitHead, GitObjectError> {
        log::debug!("Parse the HEAD file for the project");

        let content = content.trim();

        match content.split_once(" ") {
            Some(("ref:", value)) => {
                let (tag, value) = value.split_once("/").ok_or(GitObjectError::ParsingError)?;

                if tag != GitFolders::REFS.as_ref() {
                    log::debug!("Project does not have REFS folder");

                    Err(GitObjectError::ParsingError)
                } else {
                    let (tag, value) = value.split_once("/").ok_or(GitObjectError::ParsingError)?;

                    log::debug!("HEAD is set to {value}");

                    Ok(GitHead::Branch(
                        GitRefs::from(tag).ok_or(GitObjectError::ParsingError)?,
                        value.to_string(),
                    ))
                }
            }
            None => {
                if content.len() == HASH_SIZE * 2 {
                    log::debug!("HEAD is set to hash {content}");

                    Ok(GitHead::Hash(content.to_string()))
                } else {
                    log::debug!("HEAD hash is of invalid length");

                    Err(GitObjectError::ParsingError)
                }
            }
            _ => Err(GitObjectError::ParsingError),
        }
    }

    /**
     * Get the branch name if the GitHead is a branch
     */
    pub fn get_branch(&self) -> Option<(GitRefs, String)> {
        match self {
            GitHead::Branch(branch_type, branch) => Some((branch_type.clone(), branch.clone())),
            _ => None,
        }
    }

    /**
     * Get the hash if the GitHead is a hash
     */
    pub fn get_hash(&self) -> Option<String> {
        match self {
            GitHead::Hash(hash) => Some(hash.clone()),
            _ => None,
        }
    }

    /**
     * Check if the GitHead is a branch
     */
    pub fn is_branch(&self) -> bool {
        matches!(self, GitHead::Branch(_, _))
    }

    /**
     * Check if the GitHead is a hash
     */
    pub fn is_hash(&self) -> bool {
        matches!(self, GitHead::Hash(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let head = GitHead::from("ref: refs/heads/main").unwrap();
        assert!(head.is_branch());
        assert_eq!(
            head.get_branch().unwrap(),
            (GitRefs::HEADS, "main".to_string())
        );

        let head = GitHead::from("ref: refs/remotes/origin/main").unwrap();
        assert!(head.is_branch());
        assert!(!head.is_hash());
        assert_eq!(head.get_hash(), None);

        let head = GitHead::from("ref: refs/tags/v1.0.0").unwrap();
        assert_eq!(
            head.get_branch().unwrap(),
            (GitRefs::TAGS, "v1.0.0".to_string())
        );

        let head = GitHead::from("9daeafb9864cf43055ae93beb0afd6c7d144bfa4").unwrap();
        assert_eq!(
            head,
            GitHead::Hash("9daeafb9864cf43055ae93beb0afd6c7d144bfa4".to_string())
        );
        assert!(head.is_hash());

        let head = GitHead::from("1234567890abcdef1234567890abcdef12345678901");
        assert!(head.is_err());

        let head = GitHead::from("refs/heads/main");
        assert!(head.is_err());

        let head = GitHead::from("refsss: refs/remotes/origin/main");
        assert!(head.is_err());

        let head = GitHead::from("ref: nu/remotes/origin/main/");
        assert!(head.is_err());

        let head = GitHead::from("ref: refs/oraijin/origin/main/");
        assert!(head.is_err());
    }
}
