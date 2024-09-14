use crate::errors::git_object_error::GitObjectError;

use super::object::{GitObject, Header};

#[derive(Debug, Clone, PartialEq)]
pub enum GitTreeMode {
    File,
    Executable,
    Symlink,
    Tree,
    Submodule,
}

impl GitTreeMode {
    pub fn from_mode_str(mode: &str) -> Self {
        match mode {
            "100644" => GitTreeMode::File,
            "100755" => GitTreeMode::Executable,
            "120000" => GitTreeMode::Symlink,
            "040000" => GitTreeMode::Tree,
            "160000" => GitTreeMode::Submodule,
            _ => panic!("Invalid mode: {}", mode),
        }
    }

    pub fn to_mode_str(&self) -> &str {
        match self {
            GitTreeMode::File => "100644",
            GitTreeMode::Executable => "100755",
            GitTreeMode::Symlink => "120000",
            GitTreeMode::Tree => "040000",
            GitTreeMode::Submodule => "160000",
        }
    }
}

impl std::fmt::Display for GitTreeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitTreeMode::Tree => write!(f, "tree"),
            _ => write!(f, "blob"),
        }
    }
}

pub struct GitTreeEntry {
    pub mode: GitTreeMode,
    pub hash: String,
    pub name: String,
}

pub struct GitTree {
    entries: Vec<GitTreeEntry>,
}

impl Default for GitTree {
    fn default() -> Self {
        Self::new()
    }
}

impl GitTree {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, mode: GitTreeMode, hash: String, name: String) {
        self.entries.push(GitTreeEntry { mode, hash, name });
    }

    pub fn entries(&self) -> &Vec<GitTreeEntry> {
        &self.entries
    }

    pub fn get_entry_by_name(&self, name: &str) -> Option<&GitTreeEntry> {
        self.entries.iter().find(|entry| entry.name == name)
    }

    pub fn get_entry_by_hash(&self, hash: &str) -> Option<&GitTreeEntry> {
        self.entries.iter().find(|entry| entry.hash == hash)
    }

    pub fn get_trees(&self) -> Vec<&GitTreeEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.mode == GitTreeMode::Tree)
            .collect()
    }

    pub fn get_blobs(&self) -> Vec<&GitTreeEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.mode != GitTreeMode::Tree)
            .collect()
    }
}

impl GitObject for GitTree {
    fn get_type(&self) -> Header {
        Header::Tree
    }

    fn get_data_string(&self) -> String {
        let mut data = String::new();
        for entry in &self.entries {
            data.push_str(&format!(
                "{} {}\0{}",
                entry.mode.to_mode_str(),
                entry.name,
                entry.hash
            ));
        }

        data
    }

    fn from_encoded_data(encoded_data: &[u8]) -> Result<Self, GitObjectError>
    where
        Self: Sized,
    {
        let decoded_data = Self::decode_data(encoded_data)?;
        let (data, _) = Self::check_header_valid_and_get_data(&decoded_data)?;

        let mut tree = Self::new();
        let mut data = &data[..data.len() - 1];
        while !data.is_empty() {
            let (mode, rest_object) = data
                .split_once(' ')
                .ok_or(GitObjectError::InvalidTreeFile)?;
            let (name, rest_object) = rest_object
                .split_once('\0')
                .ok_or(GitObjectError::InvalidTreeFile)?;
            let hash = rest_object
                .get(..40)
                .ok_or(GitObjectError::InvalidTreeFile)?;

            data = &rest_object[40..];

            tree.add_entry(
                GitTreeMode::from_mode_str(mode),
                hash.to_string(),
                name.to_string(),
            );
        }

        Ok(tree)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    fn create_encoded_tree_file(entries: Vec<GitTreeEntry>) -> Result<Vec<u8>, GitObjectError> {
        let mut file_content = String::new();
        for entry in entries {
            file_content.push_str(&format!(
                "{} {}\0{}",
                entry.mode.to_mode_str(),
                entry.name,
                entry.hash,
            ));
        }

        let file_content_to_encode = format!("tree {}\x00{}\n", file_content.len(), file_content);
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
    fn test_git_tree_from_encoded_data() {
        let entries = vec![
            GitTreeEntry {
                mode: GitTreeMode::File,
                hash: "df6773ea47ed3fce3b3bb14e3d1101963e77ef08".to_string(),
                name: "file1".to_string(),
            },
            GitTreeEntry {
                mode: GitTreeMode::Tree,
                hash: "df6773ea47ed3fce3b3bb14e3d1101963e77ef09".to_string(),
                name: "tree1".to_string(),
            },
        ];
        let encoded_data = create_encoded_tree_file(entries).unwrap();

        let tree = GitTree::from_encoded_data(encoded_data.as_slice()).unwrap();

        assert_eq!(tree.entries().len(), 2);
        assert_eq!(tree.get_blobs().len(), 1);
        assert_eq!(tree.get_trees().len(), 1);
        assert_eq!(
            tree.get_entry_by_name("file1").unwrap().hash,
            "df6773ea47ed3fce3b3bb14e3d1101963e77ef08"
        );
        assert_eq!(
            tree.get_entry_by_name("tree1").unwrap().hash,
            "df6773ea47ed3fce3b3bb14e3d1101963e77ef09"
        );
        assert_eq!(
            tree.get_entry_by_hash("df6773ea47ed3fce3b3bb14e3d1101963e77ef08")
                .unwrap()
                .name,
            "file1"
        );
    }

    #[test]
    fn test_git_tree_mode_from_mode_str() {
        assert_eq!(GitTreeMode::from_mode_str("100644"), GitTreeMode::File);
        assert_eq!(
            GitTreeMode::from_mode_str("100755"),
            GitTreeMode::Executable
        );
        assert_eq!(GitTreeMode::from_mode_str("120000"), GitTreeMode::Symlink);
        assert_eq!(GitTreeMode::from_mode_str("040000"), GitTreeMode::Tree);
        assert_eq!(GitTreeMode::from_mode_str("160000"), GitTreeMode::Submodule);
    }

    #[test]
    fn test_git_tree_mode_to_mode_str() {
        assert_eq!(GitTreeMode::File.to_mode_str(), "100644");
        assert_eq!(GitTreeMode::Executable.to_mode_str(), "100755");
        assert_eq!(GitTreeMode::Symlink.to_mode_str(), "120000");
        assert_eq!(GitTreeMode::Tree.to_mode_str(), "040000");
        assert_eq!(GitTreeMode::Submodule.to_mode_str(), "160000");
    }

    #[test]
    fn test_git_tree_mode_display() {
        assert_eq!(format!("{}", GitTreeMode::File), "blob");
        assert_eq!(format!("{}", GitTreeMode::Submodule), "blob");
        assert_eq!(format!("{}", GitTreeMode::Tree), "tree");
    }
}
