// This file contains the database struct and its implementation
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::sync::Mutex;

use crate::git::git_project::GitProject;

lazy_static! {
    pub static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

pub type Result<T> = std::result::Result<T, LoadError>;
#[derive(thiserror::Error, Debug)]
pub enum LoadError {
    #[error("File error")]
    FileError(#[from] std::io::Error),

    #[error("JSON error")]
    JSONError(#[from] serde_json::Error),

    #[error("Project already exists")]
    ProjectExists,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    path: String,
    test_mode: bool,
    projects: Vec<GitProject>,
    current_project: Option<GitProject>,
}

impl Database {
    /// Creates a new, empty `Database` instance with default values.
    ///
    /// The database will have no projects, no current project, an empty path, and test mode disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// let db = Database::new();
    /// assert!(db.get_projects().is_empty());
    /// assert_eq!(db.get_current_project(), None);
    /// ```
    fn new() -> Self {
        Database {
            path: String::new(),
            test_mode: false,
            projects: Vec::new(),
            current_project: None,
        }
    }

    /// Adds a new Git project to the database if it does not already exist.
    ///
    /// Returns an error if a project with the same directory is already present.
    /// On success, persists the updated project list to disk unless in test mode.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the project was added successfully.
    /// - `Err(LoadError::ProjectExists)` if a duplicate project is detected.
    /// - Other `Err(LoadError)` variants if saving fails.
    pub fn add_project(&mut self, project: GitProject) -> Result<()> {
        log::debug!("Adding project {} to the database", project.get_directory());

        // Check if the project already exists
        if self
            .projects
            .iter()
            .any(|p| p.get_directory() == project.get_directory())
        {
            log::debug!("Project already exists in the database");

            // Return an error if the project already exists
            Err(LoadError::ProjectExists)
        } else {
            // Add the project to the database and save it
            self.projects.push(project);
            self.save()?;

            Ok(())
        }
    }

    /// Removes a project from the database and persists the change.
    ///
    /// Returns an error if saving the updated database fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut db = Database::new();
    /// let project = GitProject::new("my_project_dir".to_string());
    /// db.add_project(project.clone()).unwrap();
    /// db.remove_project(project).unwrap();
    /// assert!(db.get_projects().is_empty());
    /// ```
    pub fn remove_project(&mut self, project: GitProject) -> Result<()> {
        log::debug!(
            "Removing project {} from the database",
            project.get_directory()
        );

        // Remove the project from the database and save it
        self.projects.retain(|p| p != &project);
        self.save()?;

        Ok(())
    }

    /// Returns a clone of the list of all projects currently stored in the database.
    ///
    /// # Examples
    ///
    /// ```
    /// let db = Database::new();
    /// let projects = db.get_projects();
    /// assert!(projects.is_empty());
    /// ```
    pub fn get_projects(&self) -> Vec<GitProject> {
        self.projects.clone()
    }

    /// Persists the current database state to a JSON file, unless in test mode.
    ///
    /// If `test_mode` is enabled, this function performs no file I/O and returns immediately.
    /// Otherwise, it serializes the database and writes it to the file specified by `path`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation succeeds, or a `LoadError` if serialization or file writing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut db = Database::new();
    /// db.set_path("/tmp".to_string()).unwrap();
    /// db.save().unwrap();
    /// ```
    fn save(&self) -> Result<()> {
        log::debug!("Saving the database state");

        if self.test_mode {
            return Ok(());
        }

        // Serialize the database and write it to the file
        let data = serde_json::to_string(&self)?;
        std::fs::write(self.path.clone(), data)?;
        log::debug!("Database saved to file: {}", self.path);

        Ok(())
    }

    /// Loads the database from the JSON file at the configured path, updating the in-memory project list.
    ///
    /// If `test_mode` is enabled, this function does nothing and returns immediately.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the database was loaded successfully, or a `LoadError` if reading or deserialization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut db = Database::new();
    /// db.set_path("/tmp".to_string()).unwrap();
    /// db.load().unwrap();
    /// ```
    fn load(&mut self) -> Result<()> {
        log::debug!("Load the database from file");

        if self.test_mode {
            return Ok(());
        }

        let data = &read_to_string(self.path.clone())?;

        // Deserialize the database and set the projects
        let db: Database = serde_json::from_str(data)?;
        self.projects = db.projects.clone();

        log::debug!("Database loaded");
        Ok(())
    }

    /// Sets the database file path and loads the database from the specified location.
    ///
    /// The path is set to `{path}/database.json`, and the database is loaded from this file.
    /// Returns an error if loading fails.
    #[allow(clippy::result_unit_err)]
    pub fn set_path(&mut self, path: String) -> Result<()> {
        log::debug!("Setting database path to: {path}/database.json");

        // Set the path and load the database
        self.path = format!("{path}/database.json");
        self.load()?;

        Ok(())
    }

    /// Enables or disables test mode, which bypasses file I/O operations.
    ///
    /// When test mode is enabled, database save and load operations do not interact with the filesystem. This is useful for testing purposes.
    pub fn set_test_mode(&mut self, test_mode: bool) {
        log::debug!("Setting database to test mode");

        self.test_mode = test_mode;
    }

    /// Sets the current active project.
    ///
    /// If `project` is `Some`, the specified project becomes the current project; if `None`, the current project is cleared.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut db = Database::new();
    /// let project = GitProject::new("my_project", "/tmp/my_project");
    /// db.set_current_project(Some(project.clone()));
    /// assert_eq!(db.get_current_project(), Some(project));
    /// db.set_current_project(None);
    /// assert_eq!(db.get_current_project(), None);
    /// ```
    pub fn set_current_project(&mut self, project: Option<GitProject>) {
        let project_dir = match &project {
            Some(proj) => proj.get_directory(),
            None => "null",
        };
        log::debug!("Set current project to {project_dir}");

        self.current_project = project;
    }

    /// Returns a clone of the currently active project, if one is set.
    ///
    /// # Examples
    ///
    /// ```
    /// let db = Database::new();
    /// assert!(db.get_current_project().is_none());
    /// ```
    pub fn get_current_project(&self) -> Option<GitProject> {
        self.current_project.clone()
    }

    /// Updates an existing project in the database by replacing it with the provided project.
    ///
    /// The project to update is identified by its directory. Persists the updated state to disk unless in test mode.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update and save succeed, or a `LoadError` if saving fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut db = Database::new();
    /// let project = GitProject::new("my_dir".to_string());
    /// db.add_project(project.clone()).unwrap();
    /// let updated_project = GitProject::new("my_dir".to_string()); // with updated fields
    /// db.update_project(updated_project).unwrap();
    /// ```
    pub fn update_project(&mut self, project: GitProject) -> Result<()> {
        log::debug!(
            "Update the project {} in the database",
            project.get_directory()
        );

        // Search for the project in the database and update it
        let index = self
            .projects
            .iter()
            .position(|p| p.get_directory() == project.get_directory())
            .unwrap();
        self.projects[index] = project;
        self.save()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use crate::git::git_project_state::GitProjectState;

    use super::*;

    #[test]
    fn test_database() {
        let dir = TempDir::new("test_database").expect("Failed to create temp dir");

        let mut db = Database::new();
        _ = db.set_path(dir.path().to_str().unwrap().to_string());
        let project = GitProject::new("test");
        db.add_project(project.clone())
            .expect("Failed to add project");
        let mut db2 = Database::new();
        db2.set_path(dir.path().to_str().unwrap().to_string())
            .expect("Failed to set path and load database");

        assert_eq!(db.get_projects(), db2.get_projects());
    }

    #[test]
    fn test_already_exists() {
        let dir = TempDir::new("test_database").expect("Failed to create temp dir");

        let mut db = Database::new();
        _ = db.set_path(dir.path().to_str().unwrap().to_string());
        let project = GitProject::new("test");
        db.add_project(project.clone())
            .expect("Failed to add project");
        let result = db.add_project(project.clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_project() {
        let dir = TempDir::new("test_database").expect("Failed to create temp dir");

        let mut db = Database::new();
        _ = db.set_path(dir.path().to_str().unwrap().to_string());
        let project = GitProject::new("test");
        db.add_project(project.clone())
            .expect("Failed to add project");
        db.remove_project(project.clone())
            .expect("Failed to remove project");
        let projects = db.get_projects();
        assert_eq!(projects.len(), 0);
    }

    #[test]
    fn test_test_mode() {
        let dir = TempDir::new("test_database").expect("Failed to create temp dir");

        let mut db = Database::new();
        _ = db.set_path(dir.path().to_str().unwrap().to_string());
        db.set_test_mode(true);
        assert!(db.save().is_ok());
        assert!(db.load().is_ok());
    }

    #[test]
    fn test_update_project() {
        let dir = TempDir::new("test_database").expect("Failed to create temp dir");

        let mut db = Database::new();
        _ = db.set_path(dir.path().to_str().unwrap().to_string());
        let project = GitProject::new("test");
        db.add_project(project.clone())
            .expect("Failed to add project");
        let mut project2 = project.clone();
        project2.set_state(GitProjectState::Invalid);
        db.update_project(project2.clone())
            .expect("Failed to update project");
        let projects = db.get_projects();
        assert_eq!(projects[0].get_state(), GitProjectState::Invalid);
    }
}
