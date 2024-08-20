use serde::{Deserialize, Serialize};
use std::fs;
use strum::IntoEnumIterator;

use super::{
    git_files::GitFiles,
    git_folders::{GitBranchType, GitFolders, GitRefs, GIT_FOLDER},
};
use crate::errors::git_error::GitError;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GitProjectState {
    Valid,
    Invalid,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitProject {
    directory: String,
    state: GitProjectState,
    local_branches: Vec<String>,
    remotes: Vec<String>,
    remote_branches: Vec<String>,
    tags: Vec<String>,
}

impl GitProject{
    pub fn new(directory: &str) -> GitProject {
        GitProject {
            directory: String::from(directory),
            state: GitProjectState::Invalid,
            local_branches: Vec::new(),
            remotes: Vec::new(),
            remote_branches: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn fetch_remotes_directories(&mut self) -> Result<(), GitError> {
        self.has_required_files().map_err(|_| {
            self.state = GitProjectState::Invalid;
            GitError::InvalidGitFolder
        })?;

        let remotes_dir = format!(
            "{}/{}/{}/{}",
            self.directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::REMOTES
        );

        fs::read_dir(remotes_dir)
            .map(|entries| {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let remote_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        self.remotes.push(remote_name);
                    }
                }
            })
            .map_err(|_| {
                self.state = GitProjectState::Invalid;
                GitError::CannotOpenFolder
            })?;

        Ok(())
    }

    pub fn fetch_branches(&mut self, branch_type: GitBranchType) -> Result<(), GitError> {
        self.has_required_files().map_err(|_| {
            self.state = GitProjectState::Invalid;
            GitError::InvalidGitFolder
        })?;

        let branch_dir = match &branch_type {
            GitBranchType::Local => GitRefs::HEADS.to_string(),
            GitBranchType::Remote(remote_dir) => GitRefs::REMOTES.to_string() + "/" + remote_dir,
            GitBranchType::Tags => GitRefs::TAGS.to_string(),
        };

        let mut dirs_to_check: Vec<String> = vec![format!(
            "{}/{}/{}/{}",
            self.directory,
            GIT_FOLDER,
            GitFolders::REFS.to_string(),
            branch_dir
        )];

        while let Some(current_dir) = dirs_to_check.pop() {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                let mut entries_iter = entries.into_iter();

                while let Some(Ok(entry)) = entries_iter.next() {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs_to_check.push(path.to_str().unwrap().to_string());
                    } else {
                        let branch_name = path.file_name().unwrap().to_str().unwrap().to_string();

                        let full_branch_name = if current_dir
                            != format!(
                                "{}/{}/{}/{}",
                                self.directory,
                                GIT_FOLDER,
                                GitFolders::REFS,
                                branch_dir
                            ) {
                            current_dir.replace(
                                &format!(
                                    "{}/{}/{}/{}/",
                                    self.directory,
                                    GIT_FOLDER,
                                    GitFolders::REFS,
                                    branch_dir
                                ),
                                "",
                            ) + "/"
                                + &branch_name
                        } else {
                            branch_name
                        };

                        match &branch_type {
                            GitBranchType::Local => self.local_branches.push(full_branch_name),
                            GitBranchType::Remote(upstream) => self
                                .remote_branches
                                .push(format!("{}/{}", upstream, full_branch_name)),
                            GitBranchType::Tags => {
                                self.tags.push(format!("tags/{}", full_branch_name))
                            }
                        }
                    }
                }
            } else {
                self.state = GitProjectState::Invalid;
                return Err(GitError::CannotOpenFolder);
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
        self.state.clone()
    }

    pub fn get_local_branches(&self) -> &Vec<String> {
        &self.local_branches
    }

    pub fn get_remote_upstreams(&self) -> &Vec<String> {
        &self.remotes
    }

    pub fn get_remote_branches(&self) -> &Vec<String> {
        &self.remote_branches
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}
