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
}
