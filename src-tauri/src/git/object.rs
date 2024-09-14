use std::{io::Read, path::PathBuf};

use flate2::read::ZlibDecoder;

use crate::errors::git_object_error::{GitObjectError, ObjectError};

use super::{
    git_folders::{GitFolders, GIT_FOLDER},
    git_project::GitProject,
};

pub const HASH_SIZE: usize = 20;

pub enum HEADER {
    TREE,
    COMMIT,
    BLOB,
    INVALID,
}

impl From<&str> for HEADER {
    fn from(header: &str) -> Self {
        match header {
            "tree" => HEADER::TREE,
            "commit" => HEADER::COMMIT,
            "blob" => HEADER::BLOB,
            _ => HEADER::INVALID,
        }
    }
}

pub trait GitObject {
    fn from_hash(project: &GitProject, hash: &str) -> Result<Self, GitObjectError>
    where
        Self: Sized,
    {
        let file_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFolders::OBJECTS.to_string())
            .join(&hash[..2])
            .join(&hash[2..]);

        let data = std::fs::read(file_path).map_err(|_| GitObjectError::FileReadError)?;
        Self::from_encoded_data(data.as_slice())
    }

    fn decode_data(encoded_data: &[u8]) -> Result<String, GitObjectError> {
        let mut zlib = ZlibDecoder::new(encoded_data);
        let mut decoded_data = String::new();

        zlib.read_to_string(&mut decoded_data)
            .map_err(|_| GitObjectError::DecompressionError)?;

        Ok(decoded_data)
    }

    fn check_header_valid_and_get_data(
        decoded_data: &str,
    ) -> Result<(&str, usize), GitObjectError> {
        let (header_data, other_data) =
            decoded_data
                .split_once("\0")
                .ok_or(GitObjectError::InvalidObjectFile(
                    ObjectError::InvalidHeader,
                ))?;
        let (header, size) =
            header_data
                .split_once(" ")
                .ok_or(GitObjectError::InvalidObjectFile(
                    ObjectError::InvalidHeader,
                ))?;

        match HEADER::from(header) {
            HEADER::TREE => {}
            HEADER::COMMIT => {}
            HEADER::BLOB => {}
            HEADER::INVALID => {
                return Err(GitObjectError::InvalidObjectFile(
                    ObjectError::InvalidHeader,
                ))
            }
        }

        let size = size
            .parse()
            .map_err(|_| GitObjectError::InvalidObjectFile(ObjectError::InvalidHeader))?;

        Ok((other_data, size))
    }

    fn from_encoded_data(encoded_data: &[u8]) -> Result<Self, GitObjectError>
    where
        Self: Sized;
}
