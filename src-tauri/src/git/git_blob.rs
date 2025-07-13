use serde::{Deserialize, Serialize};

use crate::errors::git_object_error::GitObjectError;

use super::object::{GitObject, Header};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

    fn check_header_valid_blob(data: &[u8]) -> Result<(&[u8], usize), GitObjectError> {
        // Find the position of the first null byte
        if let Some(null) = data.iter().position(|&b| b == 0) {
            // Attempt to decode the header as UTF-8
            if let Ok(header) = std::str::from_utf8(&data[..null]) {
                // Header must start with blob
                if let Some(size_str) = header.strip_prefix("blob ") {
                    // The size part should be all digits
                    if !size_str.is_empty() && size_str.chars().all(|c| c.is_ascii_digit()) {
                        return Ok((&data[null + 1..], str::parse(size_str).unwrap_or(0)));
                    }
                }
            }
        }
        Err(GitObjectError::InvalidTreeFile)
    }
}

impl GitObject for GitBlob {
    /**
     * Create a new GitBlob from the encoded data
     *
     * encoded_data: The encoded data to create the GitBlob from
     *
     * Returns the GitBlob
     */
    fn from_encoded_data(
        encoded_data: &[u8],
        needs_decoding: bool,
    ) -> Result<Self, GitObjectError> {
        // Decode the data and check if the header is valid
        let decoded_data = if needs_decoding {
            &Self::decode_data(encoded_data)?
        } else {
            encoded_data
        };

        let (data, size) = if needs_decoding {
            Self::check_header_valid_blob(decoded_data)?
        } else {
            (decoded_data, decoded_data.len())
        };

        Ok(Self::new(size, data.to_vec()))
    }

    fn get_type(&self) -> Header {
        Header::Blob
    }

    /**
     * Get the data of the GitBlob as a string
     * A GitBlob is a binary file, so the data is returned as a string
     *
     * Returns the data as a string
     */
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

        let blob = GitBlob::from_encoded_data(encoded_data.as_slice(), true).unwrap();

        assert_eq!(blob.get_hash(), "9daeafb9864cf43055ae93beb0afd6c7d144bfa4");
    }

    #[test]
    fn test_git_blob_from_encoded_data() {
        let data = String::from("test");
        let encoded_data = create_encoded_blob_file(Some(data.clone())).unwrap();

        let blob = GitBlob::from_encoded_data(encoded_data.as_slice(), true).unwrap();

        assert_eq!(blob.size(), data.len());
        assert_eq!(blob.data(), data.as_bytes());
    }

    #[test]
    fn test_git_blob_from_encoded_data_invalid_blob_file() {
        let result = GitBlob::from_encoded_data(vec![0, 1, 2, 3, 4, 5].as_slice(), true);
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

        let result = GitBlob::from_encoded_data(encoded_file_content.as_slice(), true);
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
    fn test_already_decoded_data() {
        let data = vec![1, 2, 3, 4, 5];
        let blob = GitBlob::new(data.len(), data.clone());
        let decoded_data = blob.get_data_string() + "\n";

        let git_blob = GitBlob::from_encoded_data(decoded_data.as_bytes(), false).unwrap();
        assert_eq!(git_blob.get_hash(), blob.get_hash());
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
