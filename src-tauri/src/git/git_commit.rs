use std::io::Read;

use flate2::bufread::ZlibDecoder;

use crate::errors::git_commit_error::GitCommitError;

use super::git_commit_author::GitCommitAuthor;

pub struct GitCommit {
    hash: String,
    tree_hash: String,
    parent_hashes: Vec<String>,
    author: GitCommitAuthor,
    committer: GitCommitAuthor,
    message: String,
}

impl GitCommit {
    pub fn new(
        hash: String,
        tree_hash: String,
        parent_hashes: Vec<String>,
        author: GitCommitAuthor,
        committer: GitCommitAuthor,
        message: String,
    ) -> GitCommit {
        GitCommit {
            hash,
            tree_hash,
            parent_hashes,
            author,
            committer,
            message,
        }
    }

    pub fn from_string(
        commit_hash: String,
        encoded_file_content: &[u8],
    ) -> Result<GitCommit, GitCommitError> {
        let mut zlib = ZlibDecoder::new(encoded_file_content);
        let mut decoded_file_content = String::new();

        zlib.read_to_string(&mut decoded_file_content)
            .map_err(|_| GitCommitError::DecompressionError)?;

        let mut lines = decoded_file_content.lines();
        let tree_line = lines.next().ok_or(GitCommitError::InvalidCommitFile)?;
        let tree_hash = tree_line
            .strip_prefix("tree ")
            .ok_or(GitCommitError::InvalidCommitFile)?;
        let parent_hashes = lines.clone()
            .take_while(|line| line.starts_with("parent "))
            .map(|line| line.strip_prefix("parent ").unwrap().to_string())
            .collect();
        let author_line = lines.next().ok_or(GitCommitError::InvalidCommitFile)?;
        let author = GitCommitAuthor::from_string(author_line)?;
        let committer_line = lines.next().ok_or(GitCommitError::InvalidCommitFile)?;
        let committer = GitCommitAuthor::from_string(committer_line)?;
        let message = lines.collect::<Vec<&str>>().join("\n");

        Ok(GitCommit::new(
            commit_hash,
            tree_hash.to_string(),
            parent_hashes,
            author,
            committer,
            message,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::git::git_user::GitUser;

    fn create_encoded_commit_file(author: GitCommitAuthor, commiter: GitCommitAuthor, tree: Option<&str>, parent_commits: Vec<&str>, message: &str) -> Vec<u8> {
        let tree_line = match tree {
            Some(tree) => format!("tree {}\n", tree),
            None => "".to_string(),
        };
        let parent_lines = parent_commits.iter().map(|parent_commit| format!("parent {}\n", parent_commit)).collect::<Vec<String>>().join("");
        let author_line = format!("author {} <{}> {} {}\n", author.get_user().name, author.get_user().email, author.date_seconds, author.timezone);
        let commiter_line = format!("commiter {} <{}> {} {}\n", commiter.get_user().name, commiter.get_user().email, commiter.date_seconds, commiter.timezone);

        let file_content = format!("{}{}{}{}\n{}", tree_line, parent_lines, author_line, commiter_line, message);
        let file_content_to_encode = format!("commit {}\0{}", file_content.len(), file_content);

        let mut zlib = flate2::bufread::ZlibEncoder::new(file_content_to_encode.as_bytes(), flate2::Compression::default());
        let mut encoded_file_content = Vec::new();
        zlib.read_to_end(&mut encoded_file_content).unwrap();

        encoded_file_content
    }

    #[test]
    fn test_from_string() {
        let commiter = GitCommitAuthor::new(GitUser {
            name: "Andrei Serban".to_string(),
            email: "andrei.serban@brewingbytes.com".to_string()
        }, 1725372312, "+0300".to_string());

        let commit_hash = "ae575432e84a11c11b8dc3e357806f65c50f4619".to_string();
        let encoded_file_content = create_encoded_commit_file(commiter.clone(), commiter, Some("50c8353444afbef3172c999ef6cff8d31309ac3e"), Vec::new(), "test commit");

        let git_commit = GitCommit::from_string(commit_hash.clone(), &encoded_file_content).unwrap();
        assert_eq!(git_commit.hash, commit_hash);
    }

    #[test]
    fn test_new() {
        let name = "name".to_string();
        let email = "email".to_string();
        let git_user = GitUser::new(name.clone(), email.clone());
        let timezone = "timezone".to_string();
        let git_commit_author = GitCommitAuthor::new(git_user, 1, timezone.clone());
        let hash = "hash".to_string();
        let parent_hash = Vec::new();
        let message = "message".to_string();
        let git_commit = GitCommit::new(
            hash.clone(),
            hash.clone(),
            parent_hash.clone(),
            git_commit_author.clone(),
            git_commit_author.clone(),
            message.clone(),
        );
        assert_eq!(git_commit.hash, hash);
        assert_eq!(git_commit.tree_hash, hash);
        assert_eq!(git_commit.parent_hashes, parent_hash);
        assert_eq!(git_commit.author.get_user().name, name);
        assert_eq!(git_commit.author.get_user().email, email);
        assert_eq!(git_commit.committer.get_user().name, name);
        assert_eq!(git_commit.committer.get_user().email, email);
        assert_eq!(git_commit.message, message);
    }
}
