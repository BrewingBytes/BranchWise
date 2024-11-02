use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::errors::git_object_error::{CommitError, GitObjectError};

use super::git_user::GitUser;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GitCommitAuthorType {
    Author,
    Committer,
    Tagger,
}

impl Display for GitCommitAuthorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_ = match self {
            GitCommitAuthorType::Author => "author",
            GitCommitAuthorType::Committer => "committer",
            GitCommitAuthorType::Tagger => "tagger",
        };

        write!(f, "{}", type_)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitCommitAuthor {
    user: GitUser,
    pub date_seconds: i64,
    pub timezone: String,
    type_: GitCommitAuthorType,
}

impl GitCommitAuthor {
    pub fn new(
        user: GitUser,
        date_seconds: i64,
        timezone: String,
        type_: GitCommitAuthorType,
    ) -> GitCommitAuthor {
        GitCommitAuthor {
            user,
            date_seconds,
            timezone,
            type_,
        }
    }

    pub fn get_user(&self) -> &GitUser {
        &self.user
    }

    /**
     * Create a new GitCommitAuthor from a string
     *
     * Valid format: "author name <email> date_seconds timezone"
     * Valid format: "committer name <email> date_seconds timezone"
     *
     * author_line: The string to create the GitCommitAuthor from
     */
    pub fn from_string(
        author_line: &str,
        type_: GitCommitAuthorType,
    ) -> Result<GitCommitAuthor, GitObjectError> {
        // Split the author line into name, email, date_seconds, and timezone

        // First split the author line into name and the rest of the line
        // Check if the name is valid, if not return an error
        let (name, rest_line) =
            author_line
                .split_once(" <")
                .ok_or(GitObjectError::InvalidCommitFile(
                    CommitError::InvalidAuthor,
                ))?;

        // Split the rest of the line into email and the rest of the line
        // Check if the email is valid, if not return an error
        let (email, rest_line) =
            rest_line
                .split_once("> ")
                .ok_or(GitObjectError::InvalidCommitFile(
                    CommitError::InvalidAuthor,
                ))?;

        // Split the rest of the line into date_seconds and timezone
        // Check if the date_seconds is valid, if not return an error
        let (date_seconds, timezone) =
            rest_line
                .split_once(" ")
                .ok_or(GitObjectError::InvalidCommitFile(
                    CommitError::InvalidAuthor,
                ))?;

        Ok(GitCommitAuthor::new(
            GitUser::new(name.to_string(), email.to_string()),
            date_seconds
                .parse()
                .map_err(|_| GitObjectError::InvalidCommitFile(CommitError::InvalidAuthor))?,
            timezone.to_string(),
            type_,
        ))
    }

    /**
     * Convert the GitCommitAuthor to a string
     *
     * Format: "author name <email> date_seconds timezone"
     * Format: "committer name <email> date_seconds timezone"
     *
     * author: If true, the string will start with "author", if false, the string will start with "committer"
     */
    pub fn to_string(&self) -> String {
        format!(
            "{} {} <{}> {} {}",
            self.type_, self.user.name, self.user.email, self.date_seconds, self.timezone
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let git_user = GitUser::new("name".to_string(), "email".to_string());
        let git_commit_author = GitCommitAuthor::new(
            git_user,
            1,
            "timezone".to_string(),
            GitCommitAuthorType::Author,
        );
        assert_eq!(git_commit_author.get_user().name, "name".to_string());
        assert_eq!(git_commit_author.get_user().email, "email".to_string());
        assert_eq!(git_commit_author.date_seconds, 1);
        assert_eq!(git_commit_author.timezone, "timezone".to_string());
    }

    #[test]
    fn test_from_string_author() {
        let git_commit_author = GitCommitAuthor::from_string(
            "name name <email> 1 timezone",
            GitCommitAuthorType::Author,
        )
        .unwrap();
        assert_eq!(git_commit_author.get_user().name, "name name".to_string());
        assert_eq!(git_commit_author.get_user().email, "email".to_string());
        assert_eq!(git_commit_author.date_seconds, 1);
        assert_eq!(git_commit_author.timezone, "timezone".to_string());
    }

    #[test]
    fn test_from_string_commiter() {
        let git_commit_author = GitCommitAuthor::from_string(
            "name name <email> 1 timezone",
            GitCommitAuthorType::Author,
        )
        .unwrap();
        assert_eq!(git_commit_author.get_user().name, "name name".to_string());
        assert_eq!(git_commit_author.get_user().email, "email".to_string());
        assert_eq!(git_commit_author.date_seconds, 1);
        assert_eq!(git_commit_author.timezone, "timezone".to_string());
    }

    #[test]
    fn test_from_to_string_author() {
        let git_commit_author = GitCommitAuthor::from_string(
            "name name <email> 1 timezone",
            GitCommitAuthorType::Author,
        )
        .unwrap();
        assert_eq!(
            git_commit_author.to_string(),
            "author name name <email> 1 timezone".to_string()
        );
    }

    #[test]
    fn test_from_to_string_commiter() {
        let git_commit_author = GitCommitAuthor::from_string(
            "name name <email> 1 timezone",
            GitCommitAuthorType::Committer,
        )
        .unwrap();
        assert_eq!(
            git_commit_author.to_string(),
            "committer name name <email> 1 timezone".to_string()
        );
    }

    #[test]
    fn test_from_string_invalid() {
        let git_commit_author =
            GitCommitAuthor::from_string("invalid", GitCommitAuthorType::Author).unwrap_err();
        assert_eq!(
            git_commit_author,
            GitObjectError::InvalidCommitFile(CommitError::InvalidAuthor)
        );
    }

    #[test]
    fn test_serialize() {
        let git_user = GitUser::new("name".to_string(), "email".to_string());
        let git_commit_author = GitCommitAuthor::new(
            git_user,
            1,
            "timezone".to_string(),
            GitCommitAuthorType::Author,
        );
        let serialized = serde_json::to_string(&git_commit_author).unwrap();
        assert_eq!(
            serialized,
            r#"{"user":{"name":"name","email":"email"},"date_seconds":1,"timezone":"timezone","type_":"Author"}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let git_user = GitUser::new("name".to_string(), "email".to_string());
        let git_commit_author = GitCommitAuthor::new(
            git_user,
            1,
            "timezone".to_string(),
            GitCommitAuthorType::Committer,
        );
        let deserialized: GitCommitAuthor = serde_json::from_str(
            r#"{"user":{"name":"name","email":"email"},"date_seconds":1,"timezone":"timezone","type_":"Committer"}"#,
        )
        .unwrap();
        assert_eq!(deserialized, git_commit_author);
    }
}
