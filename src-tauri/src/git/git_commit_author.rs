use serde::{Deserialize, Serialize};

use crate::errors::git_object_error::GitObjectError;

use super::git_user::GitUser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitCommitAuthor {
    user: GitUser,
    pub date_seconds: i64,
    pub timezone: String,
}

impl GitCommitAuthor {
    pub fn new(user: GitUser, date_seconds: i64, timezone: String) -> GitCommitAuthor {
        GitCommitAuthor {
            user,
            date_seconds,
            timezone,
        }
    }

    pub fn get_user(&self) -> &GitUser {
        &self.user
    }

    pub fn from_string(author_line: &str) -> Result<GitCommitAuthor, GitObjectError> {
        let stripped_author_line = match author_line {
            _ if author_line.starts_with("author ") => author_line
                .strip_prefix("author ")
                .ok_or(GitObjectError::InvalidCommitFile)?,
            _ if author_line.starts_with("commiter ") => author_line
                .strip_prefix("commiter ")
                .ok_or(GitObjectError::InvalidCommitFile)?,
            _ => return Err(GitObjectError::InvalidCommitFile),
        };

        let mut split_by_email = stripped_author_line.split("<");
        let name = split_by_email
            .next()
            .ok_or(GitObjectError::InvalidCommitFile)?
            .trim();
        let mut after_email = split_by_email
            .next()
            .ok_or(GitObjectError::InvalidCommitFile)?
            .split(">");

        let email = after_email
            .next()
            .ok_or(GitObjectError::InvalidCommitFile)?
            .trim();
        let mut date_timezone = after_email
            .next()
            .ok_or(GitObjectError::InvalidCommitFile)?
            .trim()
            .split(" ");
        let date_seconds = date_timezone
            .next()
            .ok_or(GitObjectError::InvalidCommitFile)?
            .parse::<i64>()
            .map_err(|_| GitObjectError::InvalidCommitFile)?;
        let timezone = date_timezone
            .next()
            .ok_or(GitObjectError::InvalidCommitFile)?;

        Ok(GitCommitAuthor::new(
            GitUser::new(name.to_string(), email.to_string()),
            date_seconds,
            timezone.to_string(),
        ))
    }

    pub fn to_string(&self, author: bool) -> String {
        let author_or_commiter = if author { "author" } else { "commiter" };
        format!(
            "{} {} <{}> {} {}",
            author_or_commiter, self.user.name, self.user.email, self.date_seconds, self.timezone
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let git_user = GitUser::new("name".to_string(), "email".to_string());
        let git_commit_author = GitCommitAuthor::new(git_user, 1, "timezone".to_string());
        assert_eq!(git_commit_author.get_user().name, "name".to_string());
        assert_eq!(git_commit_author.get_user().email, "email".to_string());
        assert_eq!(git_commit_author.date_seconds, 1);
        assert_eq!(git_commit_author.timezone, "timezone".to_string());
    }

    #[test]
    fn test_from_string_author() {
        let git_commit_author =
            GitCommitAuthor::from_string("author name name <email> 1 timezone").unwrap();
        assert_eq!(git_commit_author.get_user().name, "name name".to_string());
        assert_eq!(git_commit_author.get_user().email, "email".to_string());
        assert_eq!(git_commit_author.date_seconds, 1);
        assert_eq!(git_commit_author.timezone, "timezone".to_string());
    }

    #[test]
    fn test_from_string_commiter() {
        let git_commit_author =
            GitCommitAuthor::from_string("commiter name name <email> 1 timezone").unwrap();
        assert_eq!(git_commit_author.get_user().name, "name name".to_string());
        assert_eq!(git_commit_author.get_user().email, "email".to_string());
        assert_eq!(git_commit_author.date_seconds, 1);
        assert_eq!(git_commit_author.timezone, "timezone".to_string());
    }

    #[test]
    fn test_from_to_string_author() {
        let git_commit_author =
            GitCommitAuthor::from_string("author name name <email> 1 timezone").unwrap();
        assert_eq!(
            git_commit_author.to_string(true),
            "author name name <email> 1 timezone".to_string()
        );
    }

    #[test]
    fn test_from_to_string_commiter() {
        let git_commit_author =
            GitCommitAuthor::from_string("commiter name name <email> 1 timezone").unwrap();
        assert_eq!(
            git_commit_author.to_string(false),
            "commiter name name <email> 1 timezone".to_string()
        );
    }

    #[test]
    fn test_from_string_invalid() {
        let git_commit_author = GitCommitAuthor::from_string("invalid").unwrap_err();
        assert_eq!(git_commit_author, GitObjectError::InvalidCommitFile);
    }

    #[test]
    fn test_serialize() {
        let git_user = GitUser::new("name".to_string(), "email".to_string());
        let git_commit_author = GitCommitAuthor::new(git_user, 1, "timezone".to_string());
        let serialized = serde_json::to_string(&git_commit_author).unwrap();
        assert_eq!(
            serialized,
            r#"{"user":{"name":"name","email":"email"},"date_seconds":1,"timezone":"timezone"}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let git_user = GitUser::new("name".to_string(), "email".to_string());
        let git_commit_author = GitCommitAuthor::new(git_user, 1, "timezone".to_string());
        let deserialized: GitCommitAuthor = serde_json::from_str(
            r#"{"user":{"name":"name","email":"email"},"date_seconds":1,"timezone":"timezone"}"#,
        )
        .unwrap();
        assert_eq!(deserialized, git_commit_author);
    }
}
