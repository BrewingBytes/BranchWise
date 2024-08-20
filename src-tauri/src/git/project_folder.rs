use super::{
    git_folders::GitBranchType,
    git_project::{GitProject, GitProjectState},
};
use crate::{database::database::DATABASE, errors::git_error::GitError};
use std::fs;

pub fn check_valid_git_project(directory: &str) -> Result<GitProject, GitError> {
    fs::read_dir(directory)
        .map(|read_dir| {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() && path.file_name().unwrap() == ".git" {
                            return Ok(GitProject::new(directory));
                        }
                    }
                    Err(_) => return Err(GitError::CannotOpenFolder),
                }
            }

            Err(GitError::NoGitFolder)
        })
        .map_err(|_| GitError::CannotOpenFolder)?
}

#[tauri::command]
pub fn open_git_project(directory: &str) -> Result<GitProject, GitError> {
    check_valid_git_project(directory).map(|mut git_project| {
        git_project.has_required_files()?;

        git_project.set_state(GitProjectState::Valid);
        let _ = git_project.fetch_branches(GitBranchType::Local);
        let _ = git_project.fetch_branches(GitBranchType::Tags);
        let _ = git_project.fetch_remotes_directories();

        for remote in git_project.get_remote_upstreams().clone() {
            let _ = git_project.fetch_branches(GitBranchType::Remote(remote));
        }

        // Add the project to the database (Note: This is not saved to disk)
        DATABASE.lock().unwrap().add_project(git_project.clone());

        Ok(git_project)
    })?
}

#[tauri::command]
pub fn save_database(app_handle: tauri::AppHandle) -> Result<(), GitError> {
    DATABASE.lock().unwrap().save(app_handle.config().as_ref()).map_err(|_| GitError::DatabaseSaveError)?;

    Ok(())
}

#[tauri::command]
pub fn get_database_projects() -> Vec<GitProject> {
    DATABASE.lock().unwrap().get_projects()
}

#[cfg(test)]
mod tests {
    use crate::git::{
        git_files::GitFiles,
        git_folders::{GitFolders, GitRefs, GIT_FOLDER},
    };
    use strum::IntoEnumIterator;
    use tempdir::TempDir;

    use super::*;

    fn create_sample_git_folder(path: &str) -> String {
        // Create a temporary directory with a .git folder
        let git_path = format!("{}/{}", path, GIT_FOLDER);
        fs::create_dir(git_path.clone()).unwrap();

        // Create all the required folders and files in the .git folder
        for folder in GitFolders::iter() {
            fs::create_dir_all(format!("{}/{}", git_path, folder)).unwrap();
        }

        for file in GitFiles::iter() {
            fs::File::create(format!("{}/{}", git_path, file)).unwrap();
        }

        for ref_folder in GitRefs::iter() {
            fs::create_dir_all(format!("{}/{}/{}", git_path, GitFolders::REFS, ref_folder))
                .unwrap();
        }

        // Return the path to the git sample project
        git_path.to_string()
    }

    fn create_local_branch(git_directory: &str, branch: &str) {
        let branch_name = branch.split('/').last().unwrap();
        let branch_directory = branch
            .split('/')
            .filter(|x| x != &branch_name)
            .collect::<Vec<&str>>()
            .join("/");

        fs::create_dir_all(format!(
            "{}/{}/{}/{}/{}",
            git_directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS,
            branch_directory
        ))
        .unwrap();

        fs::File::create(format!(
            "{}/{}/{}/{}/{}",
            git_directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS,
            branch
        ))
        .unwrap();
    }

    fn create_remote_branch(git_directory: &str, branch: &str) {
        let branch_name = branch.split('/').last().unwrap();
        let branch_directory = branch
            .split('/')
            .filter(|x| x != &branch_name)
            .collect::<Vec<&str>>()
            .join("/");

        fs::create_dir_all(format!(
            "{}/{}/{}/{}/{}",
            git_directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::REMOTES,
            branch_directory
        ))
        .unwrap();

        fs::File::create(format!(
            "{}/{}/{}/{}/{}",
            git_directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::REMOTES,
            branch
        ))
        .unwrap();
    }

    fn create_tag(git_directory: &str, tag: &str) {
        fs::File::create(format!(
            "{}/{}/{}/{}/{}",
            git_directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::TAGS,
            tag
        ))
        .unwrap();
    }

    #[test]
    fn test_get_remote_upstreams() {
        let folder = TempDir::new("test_get_remote_upstreams").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(&test_git_folder);
        create_remote_branch(&test_git_folder, "origin/main");
        create_remote_branch(&test_git_folder, "test/main");

        let mut git_project = open_git_project(&test_git_folder).unwrap();
        let _ = git_project.fetch_remotes_directories();

        assert_eq!(
            git_project
                .get_remote_upstreams()
                .contains(&"origin".to_string()),
            true
        );
        assert_eq!(
            git_project
                .get_remote_upstreams()
                .contains(&"test".to_string()),
            true
        );
    }

    #[test]
    fn test_get_remote_origin_branches() {
        let folder = TempDir::new("test_get_remote_upstreams").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(&test_git_folder);
        create_remote_branch(&test_git_folder, "origin/main");

        let mut git_project = open_git_project(&test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Remote("origin".to_string()));

        assert_eq!(
            git_project
                .get_remote_branches()
                .contains(&"origin/main".to_string()),
            true
        );
    }

    #[test]
    fn test_get_tags() {
        let folder = TempDir::new("test_get_tags").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(&test_git_folder);
        create_tag(&test_git_folder, "tag1");

        let mut git_project = open_git_project(&test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Tags);

        assert_eq!(
            git_project.get_tags().contains(&"tags/tag1".to_string()),
            true
        );
    }

    #[test]
    fn test_open_git_project_is_valid_state() {
        let folder = TempDir::new("test_open_git_project_is_valid_state").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(&test_git_folder);

        let git_project = open_git_project(&test_git_folder).unwrap();
        assert_eq!(git_project.get_state(), GitProjectState::Valid);
    }

    #[test]
    fn test_git_project_get_local_main_branch() {
        let folder = TempDir::new("test_git_project_get_local_main_branch").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(&test_git_folder);
        create_local_branch(&test_git_folder, "main");

        let mut git_project = open_git_project(&test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Local);

        assert_eq!(
            git_project
                .get_local_branches()
                .contains(&"main".to_string()),
            true
        );
    }

    #[test]
    fn test_git_project_get_local_branch_from_subfolder() {
        let folder = TempDir::new("test_git_project_get_local_branch_from_subfolder").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(&test_git_folder);
        create_local_branch(&test_git_folder, "feature/test");

        let mut git_project = open_git_project(&test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Local);

        assert_eq!(
            git_project
                .get_local_branches()
                .contains(&"feature/test".to_string()),
            true
        );
    }

    #[test]
    fn test_git_project_no_local_branches_folder() {
        let folder = TempDir::new("test_git_project_no_local_branches_folder").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

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
            .fetch_branches(GitBranchType::Local);
        assert_eq!(git_project.is_err(), true);
    }

    #[test]
    fn test_git_folder_exists() {
        let folder = TempDir::new("test_git_folder_exists").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(&test_git_folder);

        assert_eq!(check_valid_git_project(&test_git_folder).is_ok(), true);
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
        let folder = TempDir::new("test_project_has_no_git_folder").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        assert_eq!(
            open_git_project(&test_git_folder),
            Err(GitError::NoGitFolder)
        );

        fs::remove_dir_all(test_git_folder).unwrap();
    }
}
