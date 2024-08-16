use strum::IntoEnumIterator;

use crate::errors::git_error::GitError;
use crate::git::git_files::GitFiles;
use crate::git::git_folders::GitFolders;
use std::fs;

#[derive(Debug, PartialEq)]

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
}

pub fn check_valid_git_project(directory: &str) -> Result<GitProject, GitError> {
    match fs::read_dir(directory) {
        Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() && path.ends_with(".git") {
                            return Ok(GitProject::new(directory));
                        }
                    }
                    Err(_) => return Err(GitError::CannotOpenFolder),
                }
            }

            Err(GitError::NoGitFolder)
        }
        Err(_) => Err(GitError::CannotOpenFolder),
    }
}

pub fn open_git_project(directory: &str) -> Result<GitProject, GitError> {
    match check_valid_git_project(directory) {
        Ok(mut git_project) => {
            if (git_project.has_required_files()).is_err() {
                return Err(GitError::InvalidGitFolder);
            }

            git_project.state = GitProjectState::Valid;
            Ok(git_project)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_git_project_is_valid_state() {
        let git_project = open_git_project("..").unwrap();
        assert_eq!(git_project.state, GitProjectState::Valid);
    }

    #[test]
    fn test_git_folder_exists() {
        assert_eq!(check_valid_git_project(".."), Ok(GitProject::new("..")));
    }

    #[test]
    fn test_folder_is_invalid() {
        assert_eq!(
            open_git_project("nonexistent"),
            Err(GitError::CannotOpenFolder)
        );
    }

    #[test]
    fn test_project_has_no_git_folder() {
        assert_eq!(open_git_project("."), Err(GitError::NoGitFolder));
    }
}
