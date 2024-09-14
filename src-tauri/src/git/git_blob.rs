use serde::{Deserialize, Serialize};

use crate::errors::git_object_error::GitObjectError;

use super::object::{GitObject, Header};

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
}

impl GitObject for GitBlob {
    fn from_encoded_data(encoded_data: &[u8]) -> Result<Self, GitObjectError> {
        let decoded_data = Self::decode_data(encoded_data)?;
        let (data, size) = Self::check_header_valid_and_get_data(&decoded_data)?;
        let (data, _) = data.split_once("\n").ok_or(GitObjectError::ParsingError)?;

        Ok(Self::new(size, data.as_bytes().to_vec()))
    }

    fn get_type(&self) -> Header {
        Header::Blob
    }

    fn get_data_string(&self) -> String {
        self.data
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>()
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
    use crate::errors::git_object_error::ObjectError;
    use std::io::Read;

    use super::*;

    fn create_encoded_blob_file(data: Option<String>) -> Result<Vec<u8>, GitObjectError> {
        let file_content = data.unwrap_or_else(|| "test".to_string());
        let file_content_to_encode = format!("blob {}\x00{}\n", file_content.len(), file_content);

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
        let expected = format!("blob {}\x00{}", data.len(), String::from_utf8_lossy(&data));
        assert_eq!(blob.to_string(), expected);
    }

    #[test]
    fn test_hash() {
        let data = String::from("test");
        let encoded_data = create_encoded_blob_file(Some(data.clone())).unwrap();

        let blob = GitBlob::from_encoded_data(encoded_data.as_slice()).unwrap();

        assert_eq!(blob.get_hash(), "9daeafb9864cf43055ae93beb0afd6c7d144bfa4");
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
        assert_eq!(
            result,
            Err(GitObjectError::InvalidObjectFile(
                ObjectError::InvalidHeader
            ))
        );
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
