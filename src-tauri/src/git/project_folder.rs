use super::{
    git_folders::GitBranchType, git_project::GitProject, git_project_state::GitProjectState,
};
use crate::{database::storage::DATABASE, errors::git_error::GitError};
use std::fs;

pub fn check_valid_git_project(directory: &str) -> Result<GitProject, GitError> {
    fs::read_dir(directory)
        .map(|read_dir| {
            for entry in read_dir.flatten() {
                let path = entry.path();
                if path.is_dir() && path.file_name().unwrap() == ".git" {
                    return Ok(GitProject::new(directory));
                }
            }

            Err(GitError::NoGitFolder)
        })
        .map_err(|_| GitError::CannotOpenFolder)?
}

#[tauri::command]
pub fn set_current_project(project: Option<GitProject>) {
    DATABASE.lock().unwrap().set_current_project(project);
}

#[tauri::command]
pub fn open_git_project(directory: &str) -> Result<GitProject, GitError> {
    check_valid_git_project(directory).map(|mut git_project| {
        git_project.has_required_files()?;

        git_project.set_state(GitProjectState::Valid);
        let _ = git_project.fetch_branches(GitBranchType::Local);
        let _ = git_project.fetch_branches(GitBranchType::Tags);
        let _ = git_project.fetch_remotes_directories();
        let _ = git_project.fetch_packed_refs();

        for remote in git_project.get_remote_upstreams().clone() {
            let _ = git_project.fetch_branches(GitBranchType::Remote(remote));
        }

        // Add the project to the database (Note: This is not saved to disk)
        DATABASE
            .lock()
            .unwrap()
            .add_project(git_project.clone())
            .map_err(|_| GitError::DatabaseSaveError)?;

        Ok(git_project)
    })?
}

#[tauri::command]
pub fn get_database_projects() -> Vec<GitProject> {
    DATABASE.lock().unwrap().get_projects()
}

#[tauri::command]
pub fn remove_database_project(project: GitProject) -> Result<(), GitError> {
    DATABASE
        .lock()
        .unwrap()
        .remove_project(project)
        .map_err(|_| GitError::DatabaseDeleteError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use crate::git::{
        git_branch::GitBranch,
        git_commit::GitCommit,
        git_commit_author::GitCommitAuthor,
        git_files::GitFiles,
        git_folders::{GitFolders, GitRefs, GIT_FOLDER},
        git_user::GitUser,
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

        DATABASE.lock().unwrap().set_test_mode(true);

        // Return the path to the git sample project
        git_path.to_string()
    }

    fn create_local_branch(git_directory: &str, branch: &str, commit: &str) {
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
        .unwrap()
        .write_all(commit.as_bytes())
        .unwrap();
    }

    fn create_remote_branch(git_directory: &str, branch: &str, commit: &str) {
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
        .unwrap()
        .write_all(commit.as_bytes())
        .unwrap();
    }

    fn create_tag(git_directory: &str, tag: &str, commit: &str) {
        fs::File::create(format!(
            "{}/{}/{}/{}/{}",
            git_directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::TAGS,
            tag
        ))
        .unwrap()
        .write_all(commit.as_bytes())
        .unwrap();
    }

    fn create_encoded_commit_content(
        author: GitCommitAuthor,
        commiter: GitCommitAuthor,
        tree: Option<&str>,
        parent_commits: Vec<&str>,
        message: &str,
    ) -> Vec<u8> {
        let tree_line = match tree {
            Some(tree) => format!("tree {}\n", tree),
            None => "".to_string(),
        };
        let parent_lines = parent_commits
            .iter()
            .map(|parent_commit| format!("parent {}\n", parent_commit))
            .collect::<Vec<String>>()
            .join("");
        let author_line = format!(
            "author {} <{}> {} {}\n",
            author.get_user().name,
            author.get_user().email,
            author.date_seconds,
            author.timezone
        );
        let commiter_line = format!(
            "commiter {} <{}> {} {}\n",
            commiter.get_user().name,
            commiter.get_user().email,
            commiter.date_seconds,
            commiter.timezone
        );

        let file_content = format!(
            "{}{}{}{}\n{}",
            tree_line, parent_lines, author_line, commiter_line, message
        );
        let file_content_to_encode = format!("commit {}\0{}", file_content.len(), file_content);

        let mut zlib = flate2::bufread::ZlibEncoder::new(
            file_content_to_encode.as_bytes(),
            flate2::Compression::default(),
        );
        let mut encoded_file_content = Vec::new();
        zlib.read_to_end(&mut encoded_file_content).unwrap();

        encoded_file_content
    }

    fn create_commit(git_directory: &str, commit_hash: &str, commit_content: &[u8]) {
        fs::DirBuilder::new()
            .recursive(true)
            .create(format!(
                "{}/{}/{}/{}",
                git_directory,
                GIT_FOLDER,
                GitFolders::OBJECTS,
                &commit_hash[..2]
            ))
            .unwrap();

        fs::File::create(format!(
            "{}/{}/{}/{}/{}",
            git_directory,
            GIT_FOLDER,
            GitFolders::OBJECTS,
            &commit_hash[..2],
            &commit_hash[2..]
        ))
        .unwrap()
        .write_all(commit_content)
        .unwrap();
    }

    #[test]
    fn test_git_commit_from_file() {
        let folder = TempDir::new("test_git_commit_from_file").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        let author_commiter = GitCommitAuthor::new(
            GitUser::new(
                "Andrei Serban".to_string(),
                "andrei.serban@brewingbytes.com".to_string(),
            ),
            100,
            "+03:00".to_string(),
        );

        create_sample_git_folder(test_git_folder);
        let git_project = open_git_project(test_git_folder).unwrap();

        let content = create_encoded_commit_content(
            author_commiter.clone(),
            author_commiter.clone(),
            Some("tree"),
            Vec::new(),
            "test",
        );
        create_commit(git_project.get_directory(), "aabb", content.as_slice());
        let commit = GitCommit::from_file(&git_project, "aabb").unwrap();

        assert_eq!(commit.get_hash(), "aabb");
        assert_eq!(commit.get_author(), &author_commiter);
        assert_eq!(commit.get_committer(), &author_commiter);
        assert_eq!(commit.get_tree_hash(), "tree");
        assert_eq!(commit.get_parent_hashes(), &Vec::<String>::new());
        assert_eq!(commit.get_message(), "test");
    }

    #[test]
    fn test_git_commit_get_parent_commits() {
        let folder = TempDir::new("test_git_commit_get_parent_commits").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        let author_commiter = GitCommitAuthor::new(
            GitUser::new(
                "Andrei Serban".to_string(),
                "andrei.serban@brewingbytes.com".to_string(),
            ),
            100,
            "+03:00".to_string(),
        );

        create_sample_git_folder(test_git_folder);
        let git_project = open_git_project(test_git_folder).unwrap();

        let parent_content = create_encoded_commit_content(
            author_commiter.clone(),
            author_commiter.clone(),
            Some("tree"),
            Vec::new(),
            "parent",
        );
        create_commit(
            git_project.get_directory(),
            "parent",
            parent_content.as_slice(),
        );
        let content = create_encoded_commit_content(
            author_commiter.clone(),
            author_commiter.clone(),
            Some("tree"),
            vec!["parent"],
            "test",
        );
        create_commit(git_project.get_directory(), "aabb", content.as_slice());
        let commit = GitCommit::from_file(&git_project, "aabb").unwrap();

        let parent_commits = commit.get_parent_commits(&git_project).unwrap();
        assert_eq!(parent_commits.len(), 1);

        let parent_commit = parent_commits.first().unwrap();
        assert_eq!(parent_commit.get_hash(), "parent");
        assert_eq!(parent_commit.get_author(), &author_commiter);
        assert_eq!(parent_commit.get_committer(), &author_commiter);
        assert_eq!(parent_commit.get_tree_hash(), "tree");
        assert_eq!(parent_commit.get_parent_hashes(), &Vec::<String>::new());
        assert_eq!(parent_commit.get_message(), "parent");
    }

    #[test]
    fn test_get_database_projects() {
        let folder = TempDir::new("test_get_database_projects").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);

        let git_project = open_git_project(test_git_folder).unwrap();
        let projects = get_database_projects();

        assert!(projects.iter().any(|x| x == &git_project));
    }

    #[test]
    fn test_remove_database_project() {
        let folder = TempDir::new("test_remove_database_project").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);

        let git_project = open_git_project(test_git_folder).unwrap();
        let projects = get_database_projects();

        assert!(projects.iter().any(|x| x == &git_project));

        let _ = remove_database_project(git_project.clone());

        let projects = get_database_projects();

        assert!(!projects.iter().any(|x| x == &git_project));
    }

    #[test]
    fn test_get_remote_upstreams() {
        let folder = TempDir::new("test_get_remote_upstreams").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        create_remote_branch(test_git_folder, "origin/main", "origin_commit");
        create_remote_branch(test_git_folder, "test/main", "test_commit");

        let mut git_project = open_git_project(test_git_folder).unwrap();
        let _ = git_project.fetch_remotes_directories();

        assert!(git_project
            .get_remote_upstreams()
            .contains(&"origin".to_string()));
        assert!(git_project
            .get_remote_upstreams()
            .contains(&"test".to_string()));
    }

    #[test]
    fn test_get_remote_origin_branches() {
        let folder = TempDir::new("test_get_remote_upstreams").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        create_remote_branch(test_git_folder, "origin/main", "origin_commit");

        let mut git_project = open_git_project(test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Remote("origin".to_string()));

        assert!(git_project.get_remote_branches().contains(&GitBranch::new(
            "origin/main".to_string(),
            "origin_commit".to_string()
        )));
    }

    #[test]
    fn test_get_tags() {
        let folder = TempDir::new("test_get_tags").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        create_tag(test_git_folder, "tag1", "tag1_commit");

        let mut git_project = open_git_project(test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Tags);

        assert!(git_project.get_tags().contains(&GitBranch::new(
            "tags/tag1".to_string(),
            "tag1_commit".to_string()
        )));
    }

    #[test]
    fn test_open_git_project_is_valid_state() {
        let folder = TempDir::new("test_open_git_project_is_valid_state").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);

        let git_project = open_git_project(test_git_folder).unwrap();
        assert_eq!(git_project.get_state(), GitProjectState::Valid);
    }

    #[test]
    fn test_git_project_get_local_main_branch() {
        let folder = TempDir::new("test_git_project_get_local_main_branch").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        create_local_branch(test_git_folder, "main", "commit");

        let mut git_project = open_git_project(test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Local);

        assert!(git_project
            .get_local_branches()
            .contains(&GitBranch::new("main".to_string(), "commit".to_string())));
    }

    #[test]
    fn test_git_project_get_local_branch_from_subfolder() {
        let folder = TempDir::new("test_git_project_get_local_branch_from_subfolder").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        create_local_branch(test_git_folder, "feature/test", "test_commit");

        let mut git_project = open_git_project(test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Local);

        assert!(git_project.get_local_branches().contains(&GitBranch::new(
            "feature/test".to_string(),
            "test_commit".to_string()
        )));
    }

    #[test]
    fn test_git_project_update() {
        let folder = TempDir::new("test_git_project_update").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        create_local_branch(test_git_folder, "feature/test", "test_commit");

        let mut git_project = open_git_project(test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Local);

        assert!(git_project.get_local_branches().contains(&GitBranch::new(
            "feature/test".to_string(),
            "test_commit".to_string()
        )));

        create_local_branch(test_git_folder, "feature/test2", "test_commit");
        git_project.update().unwrap();

        assert_eq!(git_project.get_local_branches().len(), 2);
        assert!(git_project.get_local_branches().contains(&GitBranch::new(
            "feature/test2".to_string(),
            "test_commit".to_string()
        )));
    }

    #[test]
    fn test_git_project_update_remotes() {
        let folder = TempDir::new("test_git_project_update_remotes").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        create_remote_branch(test_git_folder, "origin/main", "origin_commit");

        let mut git_project = open_git_project(test_git_folder).unwrap();
        let _ = git_project.fetch_branches(GitBranchType::Remote("origin".to_string()));

        assert!(git_project.get_remote_branches().contains(&GitBranch::new(
            "origin/main".to_string(),
            "origin_commit".to_string()
        )));

        create_remote_branch(test_git_folder, "origin/main2", "origin_commit");
        git_project.update().unwrap();

        assert_eq!(git_project.get_remote_branches().len(), 2);
        assert!(git_project.get_remote_branches().contains(&GitBranch::new(
            "origin/main2".to_string(),
            "origin_commit".to_string()
        )));
    }

    #[test]
    fn test_git_project_no_local_branches_folder() {
        let folder = TempDir::new("test_git_project_no_local_branches_folder").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        fs::remove_dir(format!(
            "{}/{}/{}/{}",
            test_git_folder,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::HEADS
        ))
        .unwrap();

        let git_project = open_git_project(test_git_folder)
            .unwrap()
            .fetch_branches(GitBranchType::Local);
        assert!(git_project.is_err());
    }

    #[test]
    fn test_git_folder_exists() {
        let folder = TempDir::new("test_git_folder_exists").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);

        assert!(check_valid_git_project(test_git_folder).is_ok());
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
            open_git_project(test_git_folder),
            Err(GitError::NoGitFolder)
        );

        fs::remove_dir_all(test_git_folder).unwrap();
    }

    #[test]
    fn test_set_current_project() {
        let folder = TempDir::new("test_set_current_project").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);

        let git_project = open_git_project(test_git_folder).unwrap();
        set_current_project(Some(git_project.clone()));

        let current_project = DATABASE.lock().unwrap().get_current_project();
        assert_eq!(current_project, Some(git_project));
    }

    #[test]
    fn test_remove_current_project() {
        let folder = TempDir::new("test_remove_current_project").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);

        let git_project = open_git_project(test_git_folder).unwrap();
        set_current_project(Some(git_project.clone()));

        let current_project = DATABASE.lock().unwrap().get_current_project();
        assert_eq!(current_project, Some(git_project));

        set_current_project(None);

        let current_project = DATABASE.lock().unwrap().get_current_project();
        assert_eq!(current_project, None);
    }

    #[test]
    fn test_packed_refs() {
        let folder = TempDir::new("test_packed_refs").unwrap();
        let test_git_folder = folder.path().to_str().unwrap();

        create_sample_git_folder(test_git_folder);
        let mut git_project = open_git_project(test_git_folder).unwrap();
        assert_eq!(git_project.get_remote_branches().len(), 0);

        let mut packed_refs = fs::File::create(format!(
            "{}/{}/{}",
            test_git_folder,
            GIT_FOLDER,
            GitFiles::PackedRefs
        ))
        .unwrap();
        packed_refs
            .write_all("test_hash refs/remotes/test\n".as_bytes())
            .unwrap();

        let _ = git_project.fetch_packed_refs();

        assert_eq!(git_project.get_remote_branches().len(), 1);
    }
}
