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

        let mut dirs_to_check: Vec<String> = vec![format!(
            "{}/{}/{}/{}",
            self.directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS
        )];

        while let Some(current_dir) = dirs_to_check.pop() {
            match fs::read_dir(current_dir) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_dir() {
                                    dirs_to_check.push(path.to_str().unwrap().to_string());
                                } else {
                                    let branch_name =
                                        path.file_name().unwrap().to_str().unwrap().to_string();
                                    self.local_branches.push(branch_name);
                                }
                            }
                            Err(_) => {
                                self.state = GitProjectState::Invalid;
                                return Err(GitError::CannotOpenFolder);
                            }
                        }
                    }
                }
                Err(_) => {
                    self.state = GitProjectState::Invalid;
                    return Err(GitError::CannotOpenFolder);
                }
            }
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
