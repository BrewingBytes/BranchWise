use serde::{Deserialize, Serialize};

use crate::git::git_project::GitProject;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GitError {
    DatabaseSaveError,
    DatabaseDeleteError,
    InvalidGitFolder,
    CannotOpenFolder,
    NoGitFolder,
    NoLocalBranches,
    PackedRefsError,
    InvalidHead,
    InvalidHistory,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitErrorProject {
    pub error: GitError,
    pub project: GitProject,
}

impl GitErrorProject {
    pub fn new(error: GitError, project: GitProject) -> GitErrorProject {
        GitErrorProject { error, project }
    }
}
