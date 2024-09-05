use crate::errors::git_commit_error::GitCommitError;

use super::git_user::GitUser;

#[derive(Clone)]
pub struct GitCommitAuthor {
    user: GitUser,
    pub date_seconds: i64,
    pub timezone: String,
}

impl GitCommitAuthor {
    pub fn new(user: GitUser, date_seconds: i64, timezone: String) -> GitCommitAuthor {
        GitCommitAuthor { user, date_seconds, timezone }
    }

    pub fn get_user(&self) -> &GitUser {
        &self.user
    }

    pub fn from_string(author_line: &str) -> Result<GitCommitAuthor, GitCommitError> {
        let mut parts = author_line.split_whitespace();
        let name = parts.next().ok_or(GitCommitError::InvalidCommitFile)?;
        let email = parts.next().ok_or(GitCommitError::InvalidCommitFile)?.strip_prefix('<').ok_or(GitCommitError::InvalidCommitFile)?.strip_suffix('>').ok_or(GitCommitError::InvalidCommitFile)?;
        let date_seconds = parts
            .next()
            .ok_or(GitCommitError::InvalidCommitFile)?
            .parse()
            .map_err(|_| GitCommitError::InvalidCommitFile)?;
        let timezone = parts.next().ok_or(GitCommitError::InvalidCommitFile)?;

        Ok(GitCommitAuthor::new(GitUser::new(name.to_string(), email.to_string()), date_seconds, timezone.to_string()))
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
    fn test_from_string() {
        let git_commit_author = GitCommitAuthor::from_string("name <email> 1 timezone").unwrap();
        assert_eq!(git_commit_author.get_user().name, "name".to_string());
        assert_eq!(git_commit_author.get_user().email, "email".to_string());
        assert_eq!(git_commit_author.date_seconds, 1);
        assert_eq!(git_commit_author.timezone, "timezone".to_string());
    }
}
