use crate::errors::git_error::GitError;
use super::git_project::{GitProject, GitProjectState};
use std::fs;

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

            git_project.set_state(GitProjectState::Valid);
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
        assert_eq!(git_project.get_state(), GitProjectState::Valid);
    }

    #[test]
    fn git_project_get_local_main_branch() {
        let mut git_project = open_git_project("..").unwrap();
        git_project.fetch_local_branches().unwrap();
        assert_eq!(*git_project.get_local_branches(), vec!["main".to_string()]);
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
