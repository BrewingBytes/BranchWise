use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GitError {
    DatabaseSaveError,
    DatabaseDeleteError,
    InvalidGitFolder,
    CannotOpenFolder,
    NoGitFolder,
    NoLocalBranches,
}
