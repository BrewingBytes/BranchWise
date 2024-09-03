use super::git_commit_author::GitCommitAuthor;

pub struct GitCommit {
    hash: String,
    parent_hash: String,
    author: GitCommitAuthor,
    committer: GitCommitAuthor,
    message: String,
}

impl GitCommit {
    pub fn new(
        hash: String,
        parent_hash: String,
        author: GitCommitAuthor,
        committer: GitCommitAuthor,
        message: String,
    ) -> GitCommit {
        GitCommit {
            hash,
            parent_hash,
            author,
            committer,
            message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::git_user::GitUser;

    #[test]
    fn test_new() {
        let name = "name".to_string();
        let email = "email".to_string();
        let git_user = GitUser::new(name.clone(), email.clone());
        let date = "date".to_string();
        let timezone = "timezone".to_string();
        let git_commit_author = GitCommitAuthor::new(git_user, date.clone(), timezone.clone());
        let hash = "hash".to_string();
        let parent_hash = "parent_hash".to_string();
        let message = "message".to_string();
        let git_commit = GitCommit::new(
            hash.clone(),
            parent_hash.clone(),
            git_commit_author.clone(),
            git_commit_author.clone(),
            message.clone(),
        );
        assert_eq!(git_commit.hash, hash);
        assert_eq!(git_commit.parent_hash, parent_hash);
        assert_eq!(git_commit.author.get_user().name, name);
        assert_eq!(git_commit.author.get_user().email, email);
        assert_eq!(git_commit.committer.get_user().name, name);
        assert_eq!(git_commit.committer.get_user().email, email);
        assert_eq!(git_commit.message, message);
    }
}
