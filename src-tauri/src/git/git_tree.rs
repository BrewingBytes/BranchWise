use crate::errors::git_object_error::GitObjectError;

use super::{
    git_blob::GitBlob,
    git_project::GitProject,
    object::{GitObject, Header},
};

#[derive(Debug, Clone, PartialEq)]
pub enum GitTreeMode {
    File,
    Executable,
    Symlink,
    Tree,
    Submodule,
}

impl GitTreeMode {
    /**
     * Create a new GitTreeMode from a mode string
     *
     * mode: The mode string to create the GitTreeMode from
     *
     * Returns the GitTreeMode
     */
    pub fn from_mode_str(mode: &str) -> Self {
        match mode {
            "100644" => GitTreeMode::File,
            "100755" => GitTreeMode::Executable,
            "120000" => GitTreeMode::Symlink,
            "040000" => GitTreeMode::Tree,
            "160000" => GitTreeMode::Submodule,
            _ => panic!("Invalid mode: {mode}"),
        }
    }

    /**
     * Get the mode string of the GitTreeMode
     *
     * Returns the mode string
     */
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

    /**
     * Add a new entry to the GitTree
     *
     * mode: The mode of the entry
     * hash: The hash of the entry
     * name: The name of the entry
     */
    pub fn add_entry(&mut self, mode: GitTreeMode, hash: String, name: String) {
        self.entries.push(GitTreeEntry { mode, hash, name });
    }

    pub fn entries(&self) -> &Vec<GitTreeEntry> {
        &self.entries
    }

    /**
     * Get the entry from the GitTree by the name of the entry
     *
     * name: The name of the entry to get
     *
     * Returns the entry
     */
    pub fn get_entry_by_name(&self, name: &str) -> Option<&GitTreeEntry> {
        self.entries.iter().find(|entry| entry.name == name)
    }

    /**
     * Get the entry from the GitTree by the hash of the entry
     *
     * hash: The hash of the entry to get
     *
     * Returns the entry
     */
    pub fn get_entry_by_hash(&self, hash: &str) -> Option<&GitTreeEntry> {
        self.entries.iter().find(|entry| entry.hash == hash)
    }

    /**
     * Get all the tree entries in the GitTree
     *
     * Returns the tree entries
     */
    pub fn get_trees(&self) -> Vec<&GitTreeEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.mode == GitTreeMode::Tree)
            .collect()
    }

    /**
     * Get all the blob entries in the GitTree
     *
     * Returns the blob entries
     */
    pub fn get_blobs(&self) -> Vec<&GitTreeEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.mode != GitTreeMode::Tree)
            .collect()
    }

    /**
     * Retrieve all blobs entries in the GitTree
     * and from the trees in the GitTree
     *
     * Returns the file name and blob entries
     */
    pub fn get_object_blobs(
        &self,
        project: &GitProject,
        folder: Option<&str>,
    ) -> Vec<(String, GitBlob)> {
        let mut objects: Vec<(String, GitBlob)> = Vec::new();

        // Add my blobs
        let _ = self.get_blobs().iter().map(|blob| {
            if let Ok(blob_obj) = GitBlob::from_hash(project, &blob.hash) {
                if let Some(folder_name) = folder {
                    objects.push((folder_name.to_string() + "/" + &blob.name, blob_obj));
                } else {
                    objects.push((blob.name.clone(), blob_obj));
                }
            }
        });

        let _ = self.get_trees().iter().map(|tree| {
            if let Ok(tree_obj) = GitTree::from_hash(project, &tree.hash) {
                let mut new_folder = tree.name.clone();
                if let Some(folder) = folder {
                    new_folder = folder.to_string() + "/" + &tree.name;
                }

                tree_obj
                    .get_object_blobs(project, Some(&new_folder))
                    .iter()
                    .for_each(|el| {
                        objects.push(el.clone());
                    });
            }
        });

        objects
    }
}

impl GitObject for GitTree {
    fn get_type(&self) -> Header {
        Header::Tree
    }

    /**
     * Get the data of the GitTree as a string
     *
     * Returns the data as a string
     */
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

    /**
     * Create a new GitTree from the encoded data
     *
     * encoded_data: The encoded data to create the GitTree from
     *
     * Returns the GitTree
     */
    fn from_encoded_data(encoded_data: &[u8], needs_decoding: bool) -> Result<Self, GitObjectError>
    where
        Self: Sized,
    {
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

        // Parse the tree entries
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

        let tree = GitTree::from_encoded_data(encoded_data.as_slice(), true).unwrap();

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
    fn test_already_decoded_data() {
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
        let tree = GitTree::from_encoded_data(encoded_data.as_slice(), true).unwrap();

        let decoded_data = tree.get_data_string() + "\n";

        let git_tree = GitTree::from_encoded_data(decoded_data.as_bytes(), false).unwrap();
        assert_eq!(git_tree.get_hash(), tree.get_hash());
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
