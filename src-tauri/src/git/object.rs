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

pub const HASH_SIZE: usize = 20;

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

    fn get_hash(&self) -> String {
        let data = self.get_data_string();
        let file_to_hash = format!("{} {}\x00{}\n", self.get_type(), data.len() + 1, data);
        let mut hasher = Sha1::new();

        hasher.update(file_to_hash.as_bytes());

        hasher
            .finalize()
            .iter()
            .fold(String::new(), |mut output: String, b| {
                let _ = write!(output, "{b:02X}");
                output.to_lowercase()
            })
    }

    fn from_hash(project: &GitProject, hash: &str) -> Result<Self, GitObjectError>
    where
        Self: Sized,
    {
        if hash.len() != HASH_SIZE * 2 {
            return Err(GitObjectError::InvalidHash);
        }

        let file_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFolders::OBJECTS.to_string())
            .join(&hash[..2])
            .join(&hash[2..]);

        let data = std::fs::read(file_path).map_err(|_| GitObjectError::FileReadError)?;
        Self::from_encoded_data(data.as_slice())
    }

    fn write_object(&self, project: &GitProject) -> Result<(), GitObjectError> {
        let data = self.get_data_string();
        let file_to_hash = format!("{} {}\x00{}\n", self.get_type(), data.len(), data);

        let mut zlib = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        zlib.write_all(file_to_hash.as_bytes())
            .map_err(|_| GitObjectError::CompressionError)?;

        let contents = zlib
            .finish()
            .map_err(|_| GitObjectError::CompressionError)?;

        let hash = self.get_hash();
        let file_path = PathBuf::from(project.get_directory())
            .join(GIT_FOLDER)
            .join(GitFolders::OBJECTS.to_string())
            .join(&hash[..2])
            .join(&hash[2..]);

        std::fs::create_dir_all(file_path.parent().unwrap())
            .map_err(|_| GitObjectError::FileReadError)?;
        std::fs::write(file_path, contents).map_err(|_| GitObjectError::FileReadError)
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

        let size = size
            .parse()
            .map_err(|_| GitObjectError::InvalidObjectFile(ObjectError::InvalidHeader))?;

        Ok((other_data, size))
    }

    fn from_encoded_data(encoded_data: &[u8]) -> Result<Self, GitObjectError>
    where
        Self: Sized;
}
