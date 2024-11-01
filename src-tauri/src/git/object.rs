use std::{
    fmt::Write,
    io::{Read, Write as IoWrite},
    path::PathBuf,
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};

use crate::errors::git_object_error::{GitObjectError, ObjectError};

use super::{
    git_folders::{GitFolders, GIT_FOLDER},
    git_project::GitProject,
};

// The size of the hash in bytes
pub const HASH_SIZE: usize = 20;

#[derive(Debug, PartialEq)]
pub enum Header {
    Tree,
    Commit,
    Blob,
    Invalid,
}

impl From<&str> for Header {
    fn from(header: &str) -> Self {
        match header {
            "tree" => Header::Tree,
            "commit" => Header::Commit,
            "blob" => Header::Blob,
            _ => Header::Invalid,
        }
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = match self {
            Header::Tree => "tree",
            Header::Commit => "commit",
            Header::Blob => "blob",
            Header::Invalid => "invalid",
        };

        write!(f, "{}", header)
    }
}

pub trait GitObject {
    fn get_type(&self) -> Header;

    fn get_data_string(&self) -> String;

    /**
     * Get the hash of the git object as a string
     *
     * Returns the hash as a string
     */
    fn get_hash(&self) -> String {
        let data = self.get_data_string();
        let file_to_hash = format!("{} {}\x00{}\n", self.get_type(), data.len() + 1, data);

        let mut hasher = Sha1::new();

        hasher.update(file_to_hash.as_bytes());

        // Convert the hash to a lowercase hex string
        hasher
            .finalize()
            .iter()
            .fold(String::new(), |mut output: String, b| {
                let _ = write!(output, "{b:02X}");
                output.to_lowercase()
            })
    }

    /**
     * Get the git object from the hash of the object
     *
     * project: The git project to get the object from
     * hash: The hash of the object
     */
    fn from_hash(project: &GitProject, hash: &str) -> Result<Self, GitObjectError>
    where
        Self: Sized,
    {
        if hash.len() != HASH_SIZE * 2 {
            return Err(GitObjectError::InvalidHash);
        }

        // The first two characters of the hash are the folder
        // The rest of the characters are the file
        let file_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFolders::OBJECTS.to_string())
            .join(&hash[..2])
            .join(&hash[2..]);

        // Read the file and get the encoded data
        let data = std::fs::read(file_path).map_err(|_| GitObjectError::FileReadError)?;
        Self::from_encoded_data(data.as_slice())
    }

    /**
     * Get the encoded data of the git object as a vector of bytes
     * format is: {type} {size}\x00{data} encoded with zlib
     *
     * Returns the encoded data as a vector of bytes or an error
     */
    fn get_encoded_data(&self) -> Result<Vec<u8>, GitObjectError> {
        let data = self.get_data_string();
        let file_to_hash = format!("{} {}\x00{}\n", self.get_type(), data.len(), data);

        let mut zlib = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        zlib.write_all(file_to_hash.as_bytes())
            .map_err(|_| GitObjectError::CompressionError)?;

        zlib.finish().map_err(|_| GitObjectError::CompressionError)
    }

    /**
     * Write the git object to the git project
     *
     * project: The git project to write the object to
     */
    fn write_object(&self, project: &GitProject) -> Result<(), GitObjectError> {
        let encoded_data = self.get_encoded_data()?;

        // Get the hash of the object and create the file path
        let hash = self.get_hash();
        let file_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFolders::OBJECTS.to_string())
            .join(&hash[..2])
            .join(&hash[2..]);

        // Create the file and write the encoded data
        std::fs::create_dir_all(file_path.parent().unwrap())
            .map_err(|_| GitObjectError::FileReadError)?;
        std::fs::write(file_path, encoded_data).map_err(|_| GitObjectError::FileReadError)
    }

    /**
     * Decode the encoded data of the git object
     *
     * encoded_data: The encoded data of the git object
     *
     * Returns the decoded data as a string or an error
     */
    fn decode_data(encoded_data: &[u8]) -> Result<String, GitObjectError> {
        let mut zlib = ZlibDecoder::new(encoded_data);
        let mut decoded_data = String::new();

        // Read the decoded data into a string from the zlib decoder
        zlib.read_to_string(&mut decoded_data)
            .map_err(|_| GitObjectError::DecompressionError)?;

        Ok(decoded_data)
    }

    /**
     * Check the header of the decoded data and get the data
     *
     * decoded_data: The decoded data of the git object
     *
     * Returns the data and the size of the data
     */
    fn check_header_valid_and_get_data(
        decoded_data: &str,
    ) -> Result<(&str, usize), GitObjectError> {
        // Split the decoded data into the header and the rest of the data
        // Check if the decoded data is valid, if not return an error
        let (header_data, other_data) =
            decoded_data
                .split_once("\0")
                .ok_or(GitObjectError::InvalidObjectFile(
                    ObjectError::InvalidHeader,
                ))?;

        // Split the header data into the header and the size of the data
        // Check if the header is valid, if not return an error
        let (header, size) =
            header_data
                .split_once(" ")
                .ok_or(GitObjectError::InvalidObjectFile(
                    ObjectError::InvalidHeader,
                ))?;

        // Check if the header name is valid, if not return an error
        match Header::from(header) {
            Header::Tree => {}
            Header::Commit => {}
            Header::Blob => {}
            Header::Invalid => {
                return Err(GitObjectError::InvalidObjectFile(
                    ObjectError::InvalidHeader,
                ))
            }
        }

        // Parse the size of the data, if it is not a number return an error
        let size = size
            .parse()
            .map_err(|_| GitObjectError::InvalidObjectFile(ObjectError::InvalidHeader))?;

        // Return the data and the size of the data
        Ok((other_data, size))
    }

    fn from_encoded_data(encoded_data: &[u8]) -> Result<Self, GitObjectError>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_header() {
        assert_eq!(Header::from("tree"), Header::Tree);
        assert_eq!(Header::from("commit"), Header::Commit);
        assert_eq!(Header::from("blob"), Header::Blob);
        assert_eq!(Header::from("other"), Header::Invalid);
    }

    #[test]
    fn test_display_header() {
        assert_eq!(Header::Tree.to_string(), "tree");
        assert_eq!(Header::Commit.to_string(), "commit");
        assert_eq!(Header::Blob.to_string(), "blob");
        assert_eq!(Header::Invalid.to_string(), "invalid");
    }
}
