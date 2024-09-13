use std::{io::Read, path::PathBuf};

use flate2::bufread::ZlibDecoder;
use serde::{Deserialize, Serialize};

use crate::errors::git_object_error::GitObjectError;

use super::{
    git_folders::{GitFolders, GIT_FOLDER},
    git_project::GitProject,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GitBlob {
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

    pub fn from_encoded_data(encoded_data: &[u8]) -> Result<Self, GitObjectError> {
        let mut zlib = ZlibDecoder::new(encoded_data);
        let mut decoded_file_content = String::new();

        zlib.read_to_string(&mut decoded_file_content)
            .map_err(|_| GitObjectError::DecompressionError)?;

        let mut lines = decoded_file_content.lines();

        let blob_line = lines.next().ok_or(GitObjectError::InvalidBlobFile)?;
        let blob_line = blob_line
            .split("\0")
            .nth(0)
            .ok_or(GitObjectError::InvalidBlobFile)?;
        let blob_size = blob_line
            .strip_prefix("blob ")
            .ok_or(GitObjectError::InvalidBlobFile)?
            .parse::<usize>()
            .map_err(|_| GitObjectError::ParsingError)?;

        let data = decoded_file_content.as_bytes()[blob_line.len() + 1..].to_vec();
        Ok(Self::new(blob_size, data))
    }

    pub fn from_hash(project: &GitProject, hash: &str) -> Result<Self, GitObjectError> {
        let file_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFolders::OBJECTS.to_string())
            .join(&hash[..2])
            .join(&hash[2..]);

        let data = std::fs::read(file_path).map_err(|_| GitObjectError::FileReadError)?;
        Self::from_encoded_data(data.as_slice())
    }
}

impl std::fmt::Display for GitBlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "blob {}\0{}",
            self.size,
            String::from_utf8_lossy(&self.data)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_encoded_blob_file(data: Option<String>) -> Result<Vec<u8>, GitObjectError> {
        let file_content = data.unwrap_or_else(|| "test".to_string());
        let file_content_to_encode = format!("blob {}\0{}", file_content.len(), file_content);

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
    fn test_to_string() {
        let data = vec![1, 2, 3, 4, 5];
        let blob = GitBlob::new(data.len(), data.clone());
        let expected = format!("blob {}\0{}", data.len(), String::from_utf8_lossy(&data));
        assert_eq!(blob.to_string(), expected);
    }

    #[test]
    fn test_git_blob_from_encoded_data() {
        let data = String::from("test");
        let encoded_data = create_encoded_blob_file(Some(data.clone())).unwrap();

        let blob = GitBlob::from_encoded_data(encoded_data.as_slice()).unwrap();

        assert_eq!(blob.size(), data.len());
        assert_eq!(blob.data(), data.as_bytes());
    }

    #[test]
    fn test_git_blob_from_encoded_data_invalid_blob_file() {
        let result = GitBlob::from_encoded_data(vec![0, 1, 2, 3, 4, 5].as_slice());
        assert_eq!(result, Err(GitObjectError::DecompressionError));
    }

    #[test]
    fn test_git_blob_from_encoded_data_parsing_error() {
        let file_content = "test";
        let file_content_to_encode = format!("bob {}\0{}", file_content.len(), file_content);

        let mut zlib = flate2::bufread::ZlibEncoder::new(
            file_content_to_encode.as_bytes(),
            flate2::Compression::default(),
        );
        let mut encoded_file_content = Vec::new();
        zlib.read_to_end(&mut encoded_file_content).unwrap();

        let result = GitBlob::from_encoded_data(encoded_file_content.as_slice());
        assert_eq!(result, Err(GitObjectError::InvalidBlobFile));
    }

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
