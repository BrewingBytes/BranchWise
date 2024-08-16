use super::git_project::{GitProject, GitProjectState};
use crate::errors::git_error::GitError;
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
    use crate::git::{
        git_files::GitFiles,
        git_folders::{GitFolders, GitRefs, GIT_FOLDER},
    };
    use strum::IntoEnumIterator;

    use super::*;

    fn create_sample_git_folder(directory: &str) {
        fs::create_dir_all(format!("{}/{}", directory, GIT_FOLDER)).unwrap();

        for folder in GitFolders::iter() {
            fs::create_dir_all(format!("{}/{}/{}", directory, GIT_FOLDER, folder)).unwrap();
        }

        for file in GitFiles::iter() {
            fs::File::create(format!("{}/{}/{}", directory, GIT_FOLDER, file)).unwrap();
        }

        for ref_folder in GitRefs::iter() {
            fs::create_dir_all(format!(
                "{}/{}/{}/{}",
                directory,
                GIT_FOLDER,
                GitFolders::REFS,
                ref_folder
            ))
            .unwrap();
        }
    }

    fn create_local_branch(directory: &str, branch: &str) {
        let branch_name = branch.split('/').last().unwrap();
        let branch_directory = branch.split('/').filter(|x| x != &branch_name).collect::<Vec<&str>>().join("/");

        fs::create_dir_all(format!(
            "{}/{}/{}/{}/{}",
            directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS,
            branch_directory
        )).unwrap();
        
        fs::File::create(format!(
            "{}/{}/{}/{}/{}",
            directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS,
            branch
        ))
        .unwrap();
    }

    #[test]
    fn test_open_git_project_is_valid_state() {
        let test_git_folder = "test_open_git_project_is_valid_state";
        create_sample_git_folder(&test_git_folder);

        let git_project = open_git_project(&test_git_folder).unwrap();
        assert_eq!(git_project.get_state(), GitProjectState::Valid);

        fs::remove_dir_all(test_git_folder).unwrap();
    }

    #[test]
    fn git_project_get_local_main_branch() {
        let test_git_folder = "git_project_get_local_main_branch";
        create_sample_git_folder(&test_git_folder);
        create_local_branch(&test_git_folder, "main");

        let mut git_project = open_git_project(&test_git_folder)
            .unwrap();
        let _ = git_project.fetch_local_branches();

        assert_eq!(git_project.get_local_branches().contains(&"main".to_string()), true);

        fs::remove_dir_all(test_git_folder).unwrap();
    }

    #[test]
    fn git_project_get_local_branch_from_subfolder() {
        let test_git_folder = "git_project_get_local_branch_from_subfolder";
        create_sample_git_folder(&test_git_folder);
        create_local_branch(&test_git_folder, "feature/test");

        let mut git_project = open_git_project(&test_git_folder)
            .unwrap();
        let _ = git_project.fetch_local_branches();

        assert_eq!(git_project.get_local_branches().contains(&"feature/test".to_string()), true);

        fs::remove_dir_all(test_git_folder).unwrap();
    }

    #[test]
    fn git_project_no_local_branches_folder() {
        let test_git_folder = "git_project_no_local_branches_folder";
        create_sample_git_folder(&test_git_folder);
        fs::remove_dir(format!(
            "{}/{}/{}/{}",
            test_git_folder,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS
        ))
        .unwrap();

        let git_project = open_git_project(&test_git_folder)
            .unwrap()
            .fetch_local_branches();
        assert_eq!(git_project.is_err(), true);

        fs::remove_dir_all(test_git_folder).unwrap();
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
