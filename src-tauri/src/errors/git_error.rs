use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GitError {
    DatabaseSaveError,
    InvalidGitFolder,
    CannotOpenFolder,
    NoGitFolder,
    NoLocalBranches,
}
