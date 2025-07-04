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
    fn new() -> Self {
        Database {
            path: String::new(),
            test_mode: false,
            projects: Vec::new(),
            current_project: None,
        }
    }

    pub fn add_project(&mut self, project: GitProject) -> Result<()> {
        // Check if the project already exists
        if self
            .projects
            .iter()
            .any(|p| p.get_directory() == project.get_directory())
        {
            // Return an error if the project already exists
            Err(LoadError::ProjectExists)
        } else {
            // Add the project to the database and save it
            self.projects.push(project);
            self.save()?;

            Ok(())
        }
    }

    pub fn remove_project(&mut self, project: GitProject) -> Result<()> {
        // Remove the project from the database and save it
        self.projects.retain(|p| p != &project);
        self.save()?;

        Ok(())
    }

    pub fn get_projects(&self) -> Vec<GitProject> {
        self.projects.clone()
    }

    fn save(&self) -> Result<()> {
        if self.test_mode {
            return Ok(());
        }

        // Serialize the database and write it to the file
        let data = serde_json::to_string(&self)?;
        std::fs::write(self.path.clone(), data)?;

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        if self.test_mode {
            return Ok(());
        }

        let data = &read_to_string(self.path.clone())?;

        // Deserialize the database and set the projects
        let db: Database = serde_json::from_str(data)?;
        self.projects = db.projects.clone();

        Ok(())
    }

    pub fn set_path(&mut self, path: String) -> Result<()> {
        // Set the path and load the database
        self.path = format!("{path}/database.json");
        self.load()?;

        Ok(())
    }

    pub fn set_test_mode(&mut self, test_mode: bool) {
        self.test_mode = test_mode;
    }

    pub fn set_current_project(&mut self, project: Option<GitProject>) {
        self.current_project = project;
    }

    pub fn get_current_project(&self) -> Option<GitProject> {
        self.current_project.clone()
    }

    pub fn update_project(&mut self, project: GitProject) -> Result<()> {
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
