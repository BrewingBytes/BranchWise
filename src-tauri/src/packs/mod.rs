use std::{fs, path::PathBuf};

use pack::get_encoded_data_from_pack;

use crate::{
    errors::git_object_error::GitObjectError,
    git::{
        git_folders::{GitFolders, GitObjects, GIT_FOLDER},
        git_project::GitProject,
    },
    packs::index::is_hash_in_index,
};

pub mod index;
pub mod pack;

enum GitPackTypes {
    Index,
    Pack,
    MTimes,
    Rev,
    Unknown,
}

impl From<&str> for GitPackTypes {
    fn from(s: &str) -> Self {
        match s {
            "idx" => GitPackTypes::Index,
            "pack" => GitPackTypes::Pack,
            "mtimes" => GitPackTypes::MTimes,
            "rev" => GitPackTypes::Rev,
            _ => GitPackTypes::Unknown,
        }
    }
}

impl AsRef<str> for GitPackTypes {
    fn as_ref(&self) -> &str {
        match self {
            GitPackTypes::Index => "idx",
            GitPackTypes::Pack => "pack",
            GitPackTypes::MTimes => "mtimes",
            GitPackTypes::Rev => "rev",
            GitPackTypes::Unknown => "unknown",
        }
    }
}

/// Retrieves the encoded data for a Git object with the specified hash from the project's pack files.
///
/// Searches all index files in the project's `.git/objects/pack` directory for the given object hash.
/// If found, returns the encoded object data from the corresponding pack file. Returns an error if the hash is not found in any index or if an error occurs while accessing the pack files.
///
/// # Returns
/// A vector of bytes containing the encoded Git object data on success, or a `GitObjectError::PackError` if the object is not found or an error occurs.
///
/// # Examples
///
/// ```
/// let encoded = get_object_encoded_data(&project, "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")?;
/// assert!(!encoded.is_empty());
/// ```
pub fn get_object_encoded_data(
    project: &GitProject,
    hash: &str,
) -> Result<Vec<u8>, GitObjectError> {
    log::debug!("Getting git object from hash {hash} from packs");

    let path = PathBuf::from(&project.get_directory())
        .join(GIT_FOLDER)
        .join(GitFolders::OBJECTS.as_ref())
        .join(GitObjects::PACK.as_ref());

    let indexes = get_all_indexes(path)?;
    for index in indexes {
        let (found, offset) = is_hash_in_index(&index, hash);

        if found {
            return Ok(get_encoded_data_from_pack(
                &index.with_extension(GitPackTypes::Pack.as_ref()),
                offset,
            ));
        }
    }

    Err(GitObjectError::PackError)
}

fn get_all_indexes(path: PathBuf) -> Result<Vec<PathBuf>, GitObjectError> {
    let mut index = Vec::<PathBuf>::new();

    fs::read_dir(path)
        .map_err(|_| GitObjectError::PackError)?
        .for_each(|entry| {
            if entry.is_err() {
                return;
            }

            let entry = entry.unwrap();
            if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                if let Some(extension) = entry.path().extension() {
                    if let GitPackTypes::Index = extension.to_str().unwrap_or_default().into() {
                        index.push(entry.path());
                    }
                }
            }
        });

    Ok(index)
}
