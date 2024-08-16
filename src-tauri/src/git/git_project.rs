use std::fs;
use strum::IntoEnumIterator;

use super::{
    git_files::GitFiles,
    git_folders::{GitFolders, GitRefs, GIT_FOLDER},
};
use crate::errors::git_error::GitError;

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

    pub fn fetch_local_branches(&mut self) -> Result<(), GitError> {
        if self.has_required_files().is_err() {
            self.state = GitProjectState::Invalid;

            return Err(GitError::InvalidGitFolder);
        }

        let refs_entries = fs::read_dir(format!(
            "{}/{}/{}/{}",
            self.directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS
        ))
        .map_err(|_| {
            self.state = GitProjectState::Invalid;

            GitError::InvalidGitFolder
        })?;

        for entry in refs_entries {
            let _ = entry.map(|x| {
                if x.path().is_file() {
                    let branch_name = x.path().file_name().unwrap().to_str().unwrap().to_string();
                    self.local_branches.push(branch_name);
                } else if x.path().is_dir() {
                    // TODO: Implement reading branches from subdirectories
                }
            });
        }

        Ok(())
    }

    pub fn has_required_files(&self) -> Result<(), GitError> {
        let mut required_git_files: Vec<String> =
            GitFiles::iter().map(|file| file.to_string()).collect();

        let mut required_git_folders: Vec<String> = GitFolders::iter()
            .map(|folder| folder.to_string())
            .collect();

        let git_folder_entries =
            fs::read_dir(format!("{}/{}", self.directory, GIT_FOLDER)).unwrap();

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

    pub fn get_local_branches(&self) -> &Vec<String> {
        &self.local_branches
    }
}
