use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::errors::git_object_error::GitObjectError;

use super::{
    git_folders::{GitFolders, GIT_FOLDER},
    git_project::GitProject,
};

#[derive(Serialize, Deserialize)]
struct GitBlob {
    size: usize,
    data: Vec<u8>,
}

impl GitBlob {
    pub fn new(size: usize, data: Vec<u8>) -> Self {
        Self { size, data }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn from_encoded_data(encoded_data: Vec<u8>) -> Result<Self, GitObjectError> {
        todo!()
    }

    pub fn from_hash(project: &GitProject, hash: &str) -> Result<Self, GitObjectError> {
        let file_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFolders::OBJECTS.to_string())
            .join(&hash[..2])
            .join(&hash[2..]);
        
        let data = std::fs::read(file_path).map_err(|_| GitObjectError::FileReadError)?;
        Self::from_encoded_data(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_blob() {
        let data = vec![1, 2, 3, 4, 5];
        let blob = GitBlob::new(data.len(), data.clone());
        assert_eq!(blob.size(), data.len());
        assert_eq!(blob.data(), data.as_slice());
    }

    #[test]
    fn test_git_blob_serialization() {
        let data = vec![1, 2, 3, 4, 5];
        let blob = GitBlob::new(data.len(), data.clone());
        let serialized = serde_json::to_string(&blob).unwrap();
        let deserialized: GitBlob = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.size(), data.len());
        assert_eq!(deserialized.data(), data.as_slice());
    }

    #[test]
    fn test_git_blob_deserialization() {
        let data = vec![1, 2, 3, 4, 5];
        let serialized = format!("{{\"size\":{},\"data\":{:?}}}", data.len(), data);
        let blob: GitBlob = serde_json::from_str(&serialized).unwrap();
        assert_eq!(blob.size(), data.len());
        assert_eq!(blob.data(), data.as_slice());
    }
}
