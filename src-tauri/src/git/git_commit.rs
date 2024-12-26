use super::{
    git_commit_author::{GitCommitAuthor, GitCommitAuthorType},
    git_files::GitFilesRequired,
    git_folders::GIT_FOLDER,
    git_project::GitProject,
    git_tree::GitTree,
    object::{GitObject, Header},
};
use crate::errors::git_object_error::{CommitError, GitObjectError};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub enum CommitPrefix {
    Tree,
    Parent,
    Author,
    Committer,
    Message,
    Signature,
    Invalid,
}

impl From<&str> for CommitPrefix {
    fn from(prefix: &str) -> Self {
        match prefix {
            "tree" => CommitPrefix::Tree,
            "parent" => CommitPrefix::Parent,
            "author" => CommitPrefix::Author,
            "committer" => CommitPrefix::Committer,
            "message" => CommitPrefix::Message,
            "gpgsig" => CommitPrefix::Signature,
            _ => CommitPrefix::Invalid,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitCommit {
    tree_hash: String,
    parent_hashes: Vec<String>,
    author: GitCommitAuthor,
    committer: GitCommitAuthor,
    message: String,
    gpg_signature: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct GitCommitWithHash {
    hash: String,

    #[serde(flatten)]
    pub commit: GitCommit,
}

impl GitCommit {
    pub fn new(
        tree_hash: &str,
        parent_hashes: &[String],
        author: GitCommitAuthor,
        committer: GitCommitAuthor,
        message: &str,
        gpg_signature: Option<String>,
    ) -> GitCommit {
        GitCommit {
            tree_hash: tree_hash.to_string(),
            parent_hashes: parent_hashes.to_vec(),
            author,
            committer,
            message: message.to_string(),
            gpg_signature,
        }
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

    pub fn get_gpg_signature(&self) -> &Option<String> {
        &self.gpg_signature
    }

    pub fn get_parent_commits(
        &self,
        project: &GitProject,
    ) -> Result<Vec<GitCommit>, GitObjectError> {
        self.parent_hashes
            .iter()
            .map(|parent_hash| GitCommit::from_hash(project, parent_hash))
            .collect()
    }

    /**
     * Get the commit with the given hash from the project
     * project: The project to get the commit from
     * length: The number of commits to get, starting from the given commit
     * hash: The hash of the commit to get
     *
     * Returns the commit history, starting from the given commit
     * If length is None, all commits are returned
     */
    pub fn get_commit_history(
        &self,
        project: &GitProject,
        length: Option<usize>,
        hash: &str,
    ) -> Result<Vec<GitCommitWithHash>, GitObjectError> {
        // UI needs to display the commit hash of the commit that was clicked on
        // So we need to add the hash of the commit to the history
        let mut commit = GitCommitWithHash {
            commit: self.clone(),
            hash: hash.to_string(),
        };

        let mut history = Vec::<GitCommitWithHash>::new();
        let length = length.unwrap_or(usize::MAX);

        loop {
            // Add the commit to the history
            history.push(commit.clone());
            if history.len() >= length {
                break;
            }

            // Get the parent commits
            let parent_commits = commit.commit.get_parent_commits(project)?;
            if parent_commits.is_empty() {
                break;
            }

            // Set the next commit to the first parent commit
            commit = GitCommitWithHash {
                commit: parent_commits[0].clone(),
                hash: commit.commit.parent_hashes[0].clone(),
            };
        }

        Ok(history)
    }

    /**
     * Checkout all the files in a commit
     */
    pub fn checkout(&self, project: &GitProject) -> Result<(), GitObjectError> {
        let files =
            GitTree::from_hash(project, self.get_tree_hash())?.get_object_blobs(project, None);

        files.iter().for_each(|file| {
            let path = PathBuf::from(project.get_directory()).join(&file.0);
            let file_in_fs = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path);

            if let Ok(mut file_in_fs) = file_in_fs {
                let _ = file_in_fs.write_all(file.1.data());
            }
        });

        let head_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFilesRequired::HEAD.as_ref());
        OpenOptions::new()
            .write(true)
            .open(head_path)
            .map_err(|_| GitObjectError::InvalidCommitFile(CommitError::InvalidContent))?
            .write_all(self.get_hash().as_bytes())
            .map_err(|_| GitObjectError::InvalidHash)?;

        Ok(())
    }
}

impl GitObject for GitCommit {
    /**
     * Create a new GitCommit from the encoded data
     *
     * encoded_data: The encoded data to create the GitCommit from
     *
     * Returns the GitCommit
     */
    fn from_encoded_data(
        encoded_data: &[u8],
        needs_decoding: bool,
    ) -> Result<Self, GitObjectError> {
        // Decode the data and check if the header is valid
        let decoded_data = if needs_decoding {
            Self::decode_data(encoded_data)?
        } else {
            String::from_utf8_lossy(encoded_data).to_string()
        };

        let data = if needs_decoding {
            Self::check_header_valid_and_get_data(&decoded_data)?.0
        } else {
            &decoded_data
        };

        // The data must contain a tree hash, either an author or committer,
        // none or more parent commits, and a message
        let mut tree = String::new();
        let mut parents = Vec::<String>::new();
        let mut author = Option::<GitCommitAuthor>::None;
        let mut committer = Option::<GitCommitAuthor>::None;
        let mut in_signature = false;
        let mut signature = String::new();

        // Remove the last newline character
        let mut data = &data[..data.len() - 1];
        while !data.is_empty() {
            // Get the next line
            let (line, remaining_data) =
                data.split_once('\n')
                    .ok_or(GitObjectError::InvalidCommitFile(
                        CommitError::InvalidContent,
                    ))?;

            // If we are in the signature section, add the line to the signature
            if in_signature {
                signature += line;
                data = remaining_data;

                if line.contains("-----END PGP SIGNATURE-----") {
                    in_signature = false;
                }

                signature += "\n";
                continue;
            }

            // Get the prefix of the line, which is the first word
            // If there is none, use "message" as the prefix
            let prefix = line.split_whitespace().next().unwrap_or("message");

            let value = line
                .strip_prefix((String::from(prefix) + " ").as_str())
                .unwrap_or(remaining_data);

            // Match the prefix and assign the value to the correct field
            // If the prefix is invalid, return an error
            // If the prefix is Author or Committer, parse the value into a GitCommitAuthor
            match CommitPrefix::from(prefix) {
                CommitPrefix::Tree => tree = value.to_string(),
                CommitPrefix::Parent => parents.push(value.to_string()),
                CommitPrefix::Author => {
                    author = Some(GitCommitAuthor::from_string(
                        value,
                        GitCommitAuthorType::Author,
                    )?)
                }
                CommitPrefix::Committer => {
                    committer = Some(GitCommitAuthor::from_string(
                        value,
                        GitCommitAuthorType::Committer,
                    )?)
                }
                CommitPrefix::Signature => {
                    in_signature = true;
                    signature = value.to_string() + "\n";
                }
                _ => break,
            }

            data = remaining_data;
        }

        // Check that the author and committer are valid
        let author = author.ok_or(GitObjectError::InvalidCommitFile(
            CommitError::InvalidHeader,
        ))?;
        let committer = committer.ok_or(GitObjectError::InvalidCommitFile(
            CommitError::InvalidHeader,
        ))?;

        Ok(GitCommit::new(
            &tree,
            &parents,
            author,
            committer,
            data.trim(),
            (!signature.is_empty()).then_some(signature),
        ))
    }

    fn get_type(&self) -> Header {
        Header::Commit
    }

    fn get_data_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for GitCommit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_hashes = self
            .parent_hashes
            .iter()
            .map(|parent_hash| format!("parent {parent_hash}\n"))
            .collect::<Vec<String>>()
            .join("");

        if self.gpg_signature.is_some() {
            write!(
                f,
                "tree {}\n{}{}\n{}\ngpgsig {}\n\n{}",
                self.tree_hash,
                parent_hashes,
                self.author,
                self.committer,
                self.gpg_signature.clone().unwrap(),
                self.message
            )
        } else {
            write!(
                f,
                "tree {}\n{}{}\n{}\n\n{}",
                self.tree_hash, parent_hashes, self.author, self.committer, self.message
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    use crate::git::git_user::GitUser;

    fn mock_git_commit_author() -> GitCommitAuthor {
        GitCommitAuthor::new(
            GitUser {
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
            },
            1234567890,
            "+0000".to_string(),
            GitCommitAuthorType::Author,
        )
    }

    fn mock_git_commit_committer() -> GitCommitAuthor {
        GitCommitAuthor::new(
            GitUser {
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
            },
            1234567890,
            "+0000".to_string(),
            GitCommitAuthorType::Committer,
        )
    }

    fn mock_git_commit() -> GitCommit {
        let author = mock_git_commit_author();
        let committer = mock_git_commit_committer();

        GitCommit::new(
            "tree_hash",
            &["parent_hash1".to_string(), "parent_hash2".to_string()],
            author,
            committer,
            "commit message",
            None,
        )
    }

    fn create_encoded_commit_file(
        author: GitCommitAuthor,
        committer: GitCommitAuthor,
        tree: Option<&str>,
        parent_commits: Vec<&str>,
        message: &str,
    ) -> Result<Vec<u8>, GitObjectError> {
        let tree_line = match tree {
            Some(tree) => format!("tree {}\n", tree),
            None => "".to_string(),
        };
        let parent_lines = parent_commits
            .iter()
            .map(|parent_commit| format!("parent {}\n", parent_commit))
            .collect::<Vec<String>>()
            .join("");
        let author_line = format!(
            "author {} <{}> {} {}\n",
            author.get_user().name,
            author.get_user().email,
            author.date_seconds,
            author.timezone
        );
        let committer_line = format!(
            "committer {} <{}> {} {}\n",
            committer.get_user().name,
            committer.get_user().email,
            committer.date_seconds,
            committer.timezone
        );

        let file_content = format!(
            "{}{}{}{}\n{}",
            tree_line, parent_lines, author_line, committer_line, message
        );
        let file_content_to_encode = format!("commit {}\x00{}\n", file_content.len(), file_content);

        let mut zlib = flate2::bufread::ZlibEncoder::new(
            file_content_to_encode.as_bytes(),
            flate2::Compression::default(),
        );
        let mut encoded_file_content = Vec::new();
        zlib.read_to_end(&mut encoded_file_content)
            .map_err(|_| GitObjectError::DecompressionError)?;

        Ok(encoded_file_content)
    }

    #[test]
    fn test_from_string() {
        let author = mock_git_commit_author();
        let committer = mock_git_commit_committer();

        let commit_hash = "90f28789cc3092bb8802acb1ca9e818dd98df342".to_string();
        let encoded_file_content = create_encoded_commit_file(
            author.clone(),
            committer.clone(),
            Some("50c8353444afbef3172c999ef6cff8d31309ac3e"),
            Vec::new(),
            "test commit\n\ntest test",
        )
        .unwrap();

        let git_commit = GitCommit::from_encoded_data(&encoded_file_content, true).unwrap();
        assert_eq!(*git_commit.get_hash(), commit_hash);
        assert_eq!(*git_commit.get_parent_hashes(), Vec::<String>::new());
        assert_eq!(
            git_commit.get_tree_hash(),
            "50c8353444afbef3172c999ef6cff8d31309ac3e"
        );
        assert_eq!(git_commit.get_message(), "test commit\n\ntest test");
        assert_eq!(*git_commit.get_author(), author);
        assert_eq!(*git_commit.get_committer(), committer);
    }

    #[test]
    fn test_from_string_invalid() {
        let encoded_file_content = "invalid content".as_bytes();

        let git_commit = GitCommit::from_encoded_data(encoded_file_content, true);
        assert!(git_commit.is_err());
    }

    #[test]
    fn test_to_string_no_parent() {
        let committer = mock_git_commit_author();

        let encoded_file_content = create_encoded_commit_file(
            committer.clone(),
            committer.clone(),
            Some("50c8353444afbef3172c999ef6cff8d31309ac3e"),
            Vec::new(),
            "test commit",
        );

        let git_commit =
            GitCommit::from_encoded_data(encoded_file_content.as_ref().unwrap(), true).unwrap();

        assert_eq!(
            git_commit.get_encoded_data().unwrap(),
            encoded_file_content.unwrap()
        );
    }

    #[test]
    fn test_to_string_with_parents() {
        let committer = mock_git_commit_author();

        let parent_commit_hash = Vec::from([
            "50c8353444afbef3172c999ef6cff8d31309ac3e",
            "50c8353444afbef3172c999ef6cff8d31309ac33",
        ]);
        let encoded_file_content = create_encoded_commit_file(
            committer.clone(),
            committer.clone(),
            Some("50c8353444afbef3172c999ef6cff8d31309ac3e"),
            parent_commit_hash.clone(),
            "test commit",
        );

        let git_commit =
            GitCommit::from_encoded_data(encoded_file_content.as_ref().unwrap(), true).unwrap();

        assert_eq!(
            git_commit.get_encoded_data().unwrap(),
            encoded_file_content.unwrap()
        );
    }

    #[test]
    fn test_gpg_sig() {
        let author = mock_git_commit_author();
        let committer = mock_git_commit_committer();
        let git_commit = GitCommit::new(
            "test",
            &[],
            author,
            committer,
            "test",
            Some(
                "-----BEGIN PGP SIGNATURE-----\n\naaaaa\n-----END PGP SIGNATURE-----\n".to_string(),
            ),
        );

        let encoded_file_content = git_commit.get_encoded_data().unwrap();
        let git_commit = GitCommit::from_encoded_data(&encoded_file_content, true).unwrap();

        assert_eq!(
            git_commit.get_gpg_signature().clone().unwrap(),
            "-----BEGIN PGP SIGNATURE-----\n\naaaaa\n-----END PGP SIGNATURE-----\n"
        );
        assert_eq!(git_commit.get_encoded_data().unwrap(), encoded_file_content);
    }

    #[test]
    fn test_from_string_with_parents() {
        let committer = mock_git_commit_author();

        let commit_hash = "daf7a8b618ed4c68f4eee690b3ef761d24643b86".to_string();
        let tree_hash = "50c8353444afbef3172c999ef6cff8d31309ac3e";
        let parent_commit_hash = Vec::from([
            "50c8353444afbef3172c999ef6cff8d31309ac3e",
            "50c8353444afbef3172c999ef6cff8d31309ac33",
        ]);
        let encoded_file_content = create_encoded_commit_file(
            committer.clone(),
            committer.clone(),
            Some(tree_hash),
            parent_commit_hash.clone(),
            "test commit",
        );

        let git_commit =
            GitCommit::from_encoded_data(encoded_file_content.as_ref().unwrap(), true).unwrap();
        assert_eq!(git_commit.get_hash(), commit_hash);
        assert_eq!(git_commit.parent_hashes, parent_commit_hash);
        assert_eq!(git_commit.tree_hash, tree_hash);
        assert_eq!(git_commit.message, "test commit");
        assert_eq!(git_commit.author, committer);
    }

    #[test]
    fn test_already_decoded_data() {
        let commit = mock_git_commit();
        let decoded_data = commit.get_data_string() + "\n";

        let git_commit = GitCommit::from_encoded_data(decoded_data.as_bytes(), false).unwrap();
        assert_eq!(git_commit.get_hash(), commit.get_hash());
    }

    #[test]
    fn test_serialize_git_commit() {
        let git_commit = mock_git_commit();
        let serialized = serde_json::to_string(&git_commit).unwrap();
        let expected = r#"{"tree_hash":"tree_hash","parent_hashes":["parent_hash1","parent_hash2"],"author":{"user":{"name":"Test User","email":"test@example.com"},"date_seconds":1234567890,"timezone":"+0000","type_":"Author"},"committer":{"user":{"name":"Test User","email":"test@example.com"},"date_seconds":1234567890,"timezone":"+0000","type_":"Committer"},"message":"commit message","gpg_signature":null}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize_git_commit() {
        let json_str = r#"{"tree_hash":"tree_hash","parent_hashes":["parent_hash1","parent_hash2"],"author":{"user":{"name":"Test User","email":"test@example.com"},"date_seconds":1234567890,"timezone":"+0000","type_":"Author"},"committer":{"user":{"name":"Test User","email":"test@example.com"},"date_seconds":1234567890,"timezone":"+0000","type_":"Committer"},"message":"commit message","gpg_signature":null}"#;
        let deserialized: GitCommit = serde_json::from_str(json_str).unwrap();
        let expected = mock_git_commit();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_deserialize_invalid_json() {
        let invalid_json_str = r#"{"hash":"hash","tree_hash":"tree_hash","parent_hashes":["parent_hash1","parent_hash2"],"author":{"user":{"name":"Test User","email":"test@example.com"},"date_seconds":1234567890,"timezone":"+0000"},"committer":{"user":{"name":"Test User","email":"test@example.com"},"date_seconds":1234567890,"timezone":"+0000"},"message":"commit message""#; // Missing closing brace
        let result: Result<GitCommit, serde_json::Error> = serde_json::from_str(invalid_json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_new() {
        let name = "name".to_string();
        let email = "email".to_string();
        let git_user = GitUser::new(name.clone(), email.clone());
        let timezone = "timezone".to_string();
        let author = GitCommitAuthor::new(
            git_user.clone(),
            1,
            timezone.clone(),
            GitCommitAuthorType::Author,
        );
        let committer = GitCommitAuthor::new(
            git_user,
            1,
            timezone.clone(),
            GitCommitAuthorType::Committer,
        );
        let hash = "4117a140fb7fa0247d619593079cc0a4ef39a8aa";
        let tree_hash = "tree_hash";
        let parent_hash = Vec::new();
        let message = "message";
        let git_commit = GitCommit::new(
            tree_hash,
            parent_hash.as_slice(),
            author.clone(),
            committer.clone(),
            message,
            None,
        );
        assert_eq!(git_commit.get_hash(), hash);
        assert_eq!(git_commit.tree_hash, tree_hash);
        assert_eq!(git_commit.parent_hashes, parent_hash);
        assert_eq!(git_commit.author.get_user().name, name);
        assert_eq!(git_commit.author.get_user().email, email);
        assert_eq!(git_commit.committer.get_user().name, name);
        assert_eq!(git_commit.committer.get_user().email, email);
        assert_eq!(git_commit.message, message);
    }
}
