use strum::IntoEnumIterator;
use std::fs;

use crate::errors::git_error::GitError;
use super::{git_folders::GitFolders, git_files::GitFiles};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GitProjectState {
    Valid,
    Invalid,
}

#[derive(Debug, PartialEq)]
pub struct GitProject<'a> {
    directory: &'a str,
    state: GitProjectState,
    local_branches: Vec<String>,
    remote_branches: Vec<String>,
}

impl GitProject<'_> {
    pub fn new(directory: &str) -> GitProject {
        GitProject {
            directory,
            state: GitProjectState::Invalid,
            local_branches: Vec::new(),
            remote_branches: Vec::new(),
        }
    }

    pub fn has_required_files(&self) -> Result<(), GitError> {
        let mut required_git_files: Vec<String> =
            GitFiles::iter().map(|file| file.to_string()).collect();

        let mut required_git_folders: Vec<String> = GitFolders::iter()
            .map(|folder| folder.to_string())
            .collect();

        let git_folder_entries = fs::read_dir(format!("{}/.git", self.directory)).unwrap();

        for entry in git_folder_entries {
            entry
                .map(|x| {
                    if x.path().is_dir() {
                        let folder_name =
                            x.path().file_name().unwrap().to_str().unwrap().to_string();

                        if required_git_folders.contains(&folder_name) {
                            required_git_folders.retain(|x| *x != folder_name);
                        }
                    } else {
                        let file_name = x.path().file_name().unwrap().to_str().unwrap().to_string();

                        if required_git_files.contains(&file_name) {
                            required_git_files.retain(|x| *x != file_name);
                        }
                    }
                })
                .map_err(|_| GitError::InvalidGitFolder)?;
        }

        if !required_git_files.is_empty() || !required_git_folders.is_empty() {
            return Err(GitError::InvalidGitFolder);
        }

        Ok(())
    }

    pub fn set_state(&mut self, state: GitProjectState) {
        self.state = state;
    }

    pub fn get_state(&self) -> GitProjectState {
        self.state
    }
}
