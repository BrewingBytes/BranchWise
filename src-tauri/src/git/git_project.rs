use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use strum::IntoEnumIterator;

use super::{
    git_branch::GitBranch,
    git_files::{GitFilesOptional, GitFilesRequired},
    git_folders::{GitBranchType, GitFolders, GitRefs, GIT_FOLDER},
    git_head::GitHead,
    git_project_state::GitProjectState,
};
use crate::errors::git_error::GitError;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitProject {
    directory: String,
    state: GitProjectState,
    head: GitHead,
    local_branches: Vec<GitBranch>,
    remotes: Vec<String>,
    remote_branches: Vec<GitBranch>,
    tags: Vec<GitBranch>,
}

impl GitProject {
    /**
     * Create a new GitProject
     *
     * # Arguments
     * directory - The directory of the git project
     * # Returns
     * A new GitProject
     */
    pub fn new(directory: &str) -> GitProject {
        GitProject {
            directory: String::from(directory),
            state: GitProjectState::Invalid,
            head: GitHead::Hash(String::new()),
            local_branches: Vec::new(),
            remotes: Vec::new(),
            remote_branches: Vec::new(),
            tags: Vec::new(),
        }
    }

    /**
     * Update the git project
     *
     * # Returns
     * Ok if the git project was updated successfully
     * Err if there was an error updating the git project
     */
    pub fn update(&mut self) -> Result<(), GitError> {
        // Clear the current branches and remotes
        self.local_branches.clear();
        self.remotes.clear();
        self.remote_branches.clear();
        self.tags.clear();

        // Fetch the branches and remotes
        self.fetch_remotes_directories()?;
        self.fetch_branches(GitBranchType::Local)?;
        self.fetch_branches(GitBranchType::Tags)?;
        self.fetch_head()?;

        // Fetch the branches for each remote
        let remotes = self.remotes.clone();
        for remote in remotes {
            self.fetch_branches(GitBranchType::Remote(remote))?;
        }

        _ = self.fetch_packed_refs(); // Should be Ok if the file doesn't exist
        self.state = GitProjectState::Valid;

        Ok(())
    }

    /**
     * Fetch the remotes directories
     *
     * # Returns
     * Ok if the remotes directories were fetched successfully
     * Err if there was an error fetching the remotes directories
     */
    pub fn fetch_remotes_directories(&mut self) -> Result<(), GitError> {
        self.has_required_files().map_err(|_| {
            self.state = GitProjectState::Invalid;
            GitError::InvalidGitFolder
        })?;

        // Get the remotes directory path
        let remotes_dir = format!(
            "{}/{}/{}/{}",
            self.directory,
            GIT_FOLDER,
            GitFolders::REFS,
            GitRefs::REMOTES
        );

        // Read the remotes directory and add the remotes to the project
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

    // Fetch the branches for the git project
    pub fn fetch_branches(&mut self, branch_type: GitBranchType) -> Result<(), GitError> {
        // Check if the git project has the required files
        // If not return an error and set the state to invalid
        self.has_required_files().map_err(|_| {
            self.state = GitProjectState::Invalid;
            GitError::InvalidGitFolder
        })?;

        // Get the directory for the branch type
        // Local branches are in the heads directory
        // Remote branches are in the remotes directory with the remote name
        // Tags are in the tags directory
        let branch_dir = match &branch_type {
            GitBranchType::Local => GitRefs::HEADS.to_string(),
            GitBranchType::Remote(remote_dir) => GitRefs::REMOTES.to_string() + "/" + remote_dir,
            GitBranchType::Tags => GitRefs::TAGS.to_string(),
        };

        // Create a list of directories to check
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

                // Iterate over the entries in the directory
                while let Some(Ok(entry)) = entries_iter.next() {
                    let path = entry.path();

                    // If the entry is a directory add it to the list of directories to check
                    if path.is_dir() {
                        dirs_to_check.push(path.to_str().unwrap().to_string());
                    } else {
                        // If the entry is a file, add the branch to the project
                        let branch_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        if branch_name == GitFilesRequired::HEAD.as_ref() {
                            continue;
                        }

                        let commit_hash = fs::read_to_string(path).unwrap();

                        // Get the full branch name, including the remote name if it is a remote branch
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

                        // Add the branch to the project based on the branch type
                        match &branch_type {
                            GitBranchType::Local => self
                                .local_branches
                                .push(GitBranch::new(full_branch_name, commit_hash)),
                            GitBranchType::Remote(upstream) => {
                                self.remote_branches.push(GitBranch::new(
                                    format!("{upstream}/{full_branch_name}"),
                                    commit_hash,
                                ))
                            }
                            GitBranchType::Tags => self
                                .tags
                                .push(GitBranch::new(full_branch_name, commit_hash)),
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

    /**
     * Fetch content from the packed refs file
     *
     * # Returns
     * Ok if the packed refs file was fetched successfully
     * Err if there was an error fetching the packed refs file
     */
    pub fn fetch_packed_refs(&mut self) -> Result<(), GitError> {
        // Read the packed refs file
        let packed_refs_path = PathBuf::from(self.get_directory())
            .join(GIT_FOLDER)
            .join(GitFilesOptional::PackedRefs.to_string());

        if let Ok(refs) = fs::read_to_string(packed_refs_path) {
            let lines = refs.lines();

            for line in lines {
                // Split the line into parts, the commit hash and the branch name
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    let commit_hash = parts[0];
                    let branch_name = parts[1];

                    // Add the branch to the project based on the branch type
                    if branch_name.starts_with("refs/heads/") {
                        let branch_name = branch_name.replace("refs/heads/", "");

                        if !self.local_branches.iter().any(|br| br.name == branch_name) {
                            self.local_branches
                                .push(GitBranch::new(branch_name, commit_hash.to_string()));
                        }
                    } else if branch_name.starts_with("refs/remotes/") {
                        let branch_name = branch_name.replace("refs/remotes/", "");

                        if !self.remote_branches.iter().any(|br| br.name == branch_name) {
                            self.remote_branches
                                .push(GitBranch::new(branch_name, commit_hash.to_string()));
                        }
                    } else if branch_name.starts_with("refs/tags/") {
                        let branch_name = branch_name.replace("refs/tags/", "");

                        if !self.tags.iter().any(|br| br.name == branch_name) {
                            self.tags
                                .push(GitBranch::new(branch_name, commit_hash.to_string()));
                        }
                    }
                }
            }

            return Ok(());
        }

        Err(GitError::PackedRefsError)
    }

    /**
     * Check if the git project has the required files
     */
    pub fn has_required_files(&self) -> Result<(), GitError> {
        // Check if the git project has the required git files from the GitFilesRequired enum
        let mut required_git_files: Vec<String> = GitFilesRequired::iter()
            .map(|file| file.to_string())
            .collect();

        // Check if the git project has the required git folders from the GitFolders enum
        let mut required_git_folders: Vec<String> = GitFolders::iter()
            .map(|folder| folder.to_string())
            .collect();

        // Read the git folder entries
        let git_folder_entries =
            fs::read_dir(format!("{}/{}", self.directory, GIT_FOLDER)).unwrap();

        // Iterate over the entries in the git folder and check if the required files and folders exist
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

        // If the required files or folders are not empty, return an error, it means the git project is invalid
        if !required_git_files.is_empty() || !required_git_folders.is_empty() {
            return Err(GitError::InvalidGitFolder);
        }

        Ok(())
    }

    /**
     * Fetch the HEAD branch
     * The HEAD branch is the current branch the git project is on
     *
     * This function expects that the HEAD file exists
     */
    pub fn fetch_head(&mut self) -> Result<(), GitError> {
        // Read the HEAD file
        let head_path = PathBuf::from(self.get_directory())
            .join(GIT_FOLDER)
            .join(GitFilesRequired::HEAD.to_string());

        if let Ok(head) = fs::read_to_string(head_path) {
            self.head = GitHead::from(&head).map_err(|_| GitError::InvalidHead)?;
        }

        Ok(())
    }

    pub fn set_state(&mut self, state: GitProjectState) {
        self.state = state;
    }

    pub fn get_state(&self) -> GitProjectState {
        self.state.clone()
    }

    pub fn get_local_branches(&self) -> &Vec<GitBranch> {
        &self.local_branches
    }

    pub fn get_remote_upstreams(&self) -> &Vec<String> {
        &self.remotes
    }

    pub fn get_remote_branches(&self) -> &Vec<GitBranch> {
        &self.remote_branches
    }

    pub fn get_tags(&self) -> &Vec<GitBranch> {
        &self.tags
    }

    pub fn get_directory(&self) -> &String {
        &self.directory
    }
}
