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
            "040000" | "40000" => GitTreeMode::Tree,
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
        self.get_blobs().iter().for_each(|blob| {
            if let Ok(blob_obj) = GitBlob::from_hash(project, &blob.hash) {
                if let Some(folder_name) = folder {
                    objects.push((folder_name.to_string() + "/" + &blob.name, blob_obj));
                } else {
                    objects.push((blob.name.clone(), blob_obj));
                }
            }
        });

        self.get_trees().iter().for_each(|tree| {
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

    fn check_header_valid_tree(data: &[u8]) -> Result<&[u8], GitObjectError> {
        // Find the position of the first null byte
        if let Some(null) = data.iter().position(|&b| b == 0) {
            // Attempt to decode the header as UTF-8
            if let Ok(header) = std::str::from_utf8(&data[..null]) {
                // Header must start with tree
                if let Some(size_str) = header.strip_prefix("tree ") {
                    // The size part should be all digits
                    if !size_str.is_empty() && size_str.chars().all(|c| c.is_ascii_digit()) {
                        return Ok(&data[null + 1..]);
                    }
                }
            }
        }
        Err(GitObjectError::InvalidTreeFile)
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
            &Self::decode_data(encoded_data)?
        } else {
            encoded_data
        };

        let data = if needs_decoding {
            Self::check_header_valid_tree(decoded_data)?
        } else {
            decoded_data
        };

        // Parse the tree entries
        let mut tree = Self::new();
        let mut data = &data[..data.len()];
        while !data.is_empty() {
            // Find the space (mode delimiter)
            let space_pos = data
                .iter()
                .position(|&b| b == b' ')
                .ok_or(GitObjectError::InvalidTreeFile)?;
            let mode = std::str::from_utf8(&data[..space_pos])
                .map_err(|_| GitObjectError::InvalidTreeFile)?;

            // Find the NULL (filename delimiter)
            let rest = &data[space_pos + 1..];
            let null_pos = rest
                .iter()
                .position(|&b| b == 0)
                .ok_or(GitObjectError::InvalidTreeFile)?;
            let name = std::str::from_utf8(&rest[..null_pos])
                .map_err(|_| GitObjectError::InvalidTreeFile)?;

            // SHA-1 Hash (20 bytes after null)
            let sha_start = null_pos + 1;
            let sha_end = sha_start + 20;
            if rest.len() < sha_end {
                return Err(GitObjectError::InvalidTreeFile);
            }

            let hash = &rest[sha_start..sha_end];
            let hash = hash.iter().map(|b| format!("{b:02x}")).collect::<String>();
            data = &rest[sha_end..];

            tree.add_entry(GitTreeMode::from_mode_str(mode), hash, name.to_string());
        }

        Ok(tree)
    }
}
