use super::git_user::GitUser;

#[derive(Clone)]
pub struct GitCommitAuthor {
    user: GitUser,
    date: String,
    timezone: String,
}

impl GitCommitAuthor {
    pub fn new(user: GitUser, date: String, timezone: String) -> GitCommitAuthor {
        GitCommitAuthor { user, date, timezone }
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
        let name = "name".to_string();
        let email = "email".to_string();
        let git_user = GitUser::new(name.clone(), email.clone());
        let date = "date".to_string();
        let timezone = "timezone".to_string();
        let git_commit_author = GitCommitAuthor::new(git_user, date.clone(), timezone.clone());
        assert_eq!(git_commit_author.get_user().name, name);
        assert_eq!(git_commit_author.get_user().email, email);
        assert_eq!(git_commit_author.date, date);
        assert_eq!(git_commit_author.timezone, timezone);
    }
}
