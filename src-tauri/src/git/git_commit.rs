use core::fmt;
use std::io::Read;
use flate2::bufread::ZlibDecoder;
use serde::{Deserialize, Serialize};
use crate::errors::git_commit_error::GitCommitError;
use super::{git_commit_author::GitCommitAuthor, git_folders::{GitFolders, GIT_FOLDER}, git_project::GitProject};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    pub fn from_file(
        project: GitProject,
        commit_hash: &str,
    ) -> Result<GitCommit, GitCommitError> {
        let objects_folder = format!("{}/{}/{}", project.get_directory(), GIT_FOLDER, GitFolders::OBJECTS);

        let file_content = std::fs::read(format!("{}/{}/{}", objects_folder, &commit_hash[..2], &commit_hash[2..])).map_err(|_| GitCommitError::FileReadError)?;
        GitCommit::from_string(commit_hash.to_string(), &file_content)
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
        let tree_line = tree_line.split("\0").nth(1).ok_or(GitCommitError::InvalidCommitFile)?;
        let tree_hash = tree_line
            .strip_prefix("tree ")
            .ok_or(GitCommitError::InvalidCommitFile)?;

        let parent_hashes = lines.clone()
            .take_while(|line| line.starts_with("parent "))
            .map(|line| line.strip_prefix("parent ").unwrap().to_string())
            .collect::<Vec<String>>();

        let mut lines = lines.skip_while(|line| line.starts_with("parent "));
        let author_line = lines.next().ok_or(GitCommitError::InvalidCommitFile)?;
        let author = GitCommitAuthor::from_string(author_line)?;

        let committer_line = lines.next().ok_or(GitCommitError::InvalidCommitFile)?;
        let committer = GitCommitAuthor::from_string(committer_line)?;

        lines.next(); // skip empty line
        let message = lines.collect::<Vec<&str>>().join("\n");

        Ok(GitCommit::new(
            commit_hash,
            tree_hash.to_string(),
            parent_hashes.clone(),
            author,
            committer,
            message,
        ))
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn get_tree_hash(&self) -> &String {
        &self.tree_hash
    }

    pub fn get_parent_hashes(&self) -> &Vec<String> {
        &self.parent_hashes
    }

    pub fn get_author(&self) -> &GitCommitAuthor {
        &self.author
    }

    pub fn get_committer(&self) -> &GitCommitAuthor {
        &self.committer
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for GitCommit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_hashes = self
            .parent_hashes
            .iter()
            .map(|parent_hash| format!("parent {}\n", parent_hash))
            .collect::<Vec<String>>()
            .join("");

        let content = format!(
            "tree {}\n{}{}\n{}\n\n{}",
            self.tree_hash,
            parent_hashes,
            self.author.to_string(true),
            self.committer.to_string(false),
            self.message
        );

        write!(
            f,
            "commit {}\0{}", content.len(), content
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    use crate::git::git_user::GitUser;
    use flate2::write::ZlibEncoder;

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
        let encoded_file_content = create_encoded_commit_file(commiter.clone(), commiter.clone(), Some("50c8353444afbef3172c999ef6cff8d31309ac3e"), Vec::new(), "test commit");

        let git_commit = GitCommit::from_string(commit_hash.clone(), &encoded_file_content).unwrap();
        assert_eq!(*git_commit.get_hash(), commit_hash);
        assert_eq!(*git_commit.get_parent_hashes(), Vec::<String>::new());
        assert_eq!(git_commit.get_tree_hash(), "50c8353444afbef3172c999ef6cff8d31309ac3e");
        assert_eq!(git_commit.get_message(), "test commit");
        assert_eq!(*git_commit.get_author(), commiter);
        assert_eq!(*git_commit.get_committer(), commiter);
    }

    #[test]
    fn test_from_string_invalid() {
        let commit_hash = "50c8353444afbef3172c999ef6cff8d31309ac3e";
        let encoded_file_content = "invalid content".as_bytes();

        let git_commit = GitCommit::from_string(commit_hash.to_string(), encoded_file_content);
        assert!(git_commit.is_err());
    }

    #[test]
    fn test_to_string_no_parent() {
        let commiter = GitCommitAuthor::new(GitUser {
            name: "Andrei Serban".to_string(),
            email: "andrei.serban@brewingbytes.com".to_string()
        }, 1725372312, "+0300".to_string());

        let commit_hash = "ae575432e84a11c11b8dc3e357806f65c50f4619".to_string();
        let encoded_file_content = create_encoded_commit_file(commiter.clone(), commiter.clone(), Some("50c8353444afbef3172c999ef6cff8d31309ac3e"), Vec::new(), "test commit");

        let git_commit = GitCommit::from_string(commit_hash.clone(), &encoded_file_content).unwrap();

        let mut zlib = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        zlib.write_all(git_commit.to_string().as_bytes()).unwrap();

        let encoded_git_commit = zlib.finish().unwrap();
        assert_eq!(encoded_git_commit, encoded_file_content);
    }

    #[test]
    fn test_to_string_with_parents() {
        let commiter = GitCommitAuthor::new(GitUser {
            name: "Andrei Serban".to_string(),
            email: "andrei.serban@brewingbytes.com".to_string()
        }, 1725372312, "+0300".to_string());

        let commit_hash = "ae575432e84a11c11b8dc3e357806f65c50f4619".to_string();
        let parent_commit_hash = Vec::from(["50c8353444afbef3172c999ef6cff8d31309ac3e", "50c8353444afbef3172c999ef6cff8d31309ac33"]);
        let encoded_file_content = create_encoded_commit_file(commiter.clone(), commiter.clone(), Some("50c8353444afbef3172c999ef6cff8d31309ac3e"), parent_commit_hash.clone(), "test commit");

        let git_commit = GitCommit::from_string(commit_hash.clone(), &encoded_file_content).unwrap();

        let mut zlib = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        zlib.write_all(git_commit.to_string().as_bytes()).unwrap();

        let encoded_git_commit = zlib.finish().unwrap();
        assert_eq!(encoded_git_commit, encoded_file_content);
    }

    #[test]
    fn test_from_string_with_parents() {
        let commiter = GitCommitAuthor::new(GitUser {
            name: "Andrei Serban".to_string(),
            email: "andrei.serban@brewingbytes.com".to_string()
        }, 1725372312, "+0300".to_string());

        let commit_hash = "ae575432e84a11c11b8dc3e357806f65c50f4619".to_string();
        let parent_commit_hash = Vec::from(["50c8353444afbef3172c999ef6cff8d31309ac3e", "50c8353444afbef3172c999ef6cff8d31309ac33"]);
        let encoded_file_content = create_encoded_commit_file(commiter.clone(), commiter.clone(), Some(&commit_hash), parent_commit_hash.clone(), "test commit");

        let git_commit = GitCommit::from_string(commit_hash.clone(), &encoded_file_content).unwrap();
        assert_eq!(git_commit.hash, commit_hash);
        assert_eq!(git_commit.parent_hashes, parent_commit_hash);
        assert_eq!(git_commit.tree_hash, commit_hash);
        assert_eq!(git_commit.message, "test commit");
        assert_eq!(git_commit.author, commiter);
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
