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
    ProjectExists
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    path: String,
    test_mode: bool,
    projects: Vec<GitProject>,
}

impl Database {
    fn new() -> Self {
        Database {
            path: String::new(),
            test_mode: false,
            projects: Vec::new(),
        }
    }

    pub fn add_project(&mut self, project: GitProject) -> Result<()> {
        if self
            .projects
            .iter()
            .any(|p| p.get_directory() == project.get_directory())
        {
            Err(LoadError::ProjectExists)
        } else {
            self.projects.push(project);
            self.save()?;

            Ok(())
        }
    }

    pub fn remove_project(&mut self, project: GitProject) -> Result<()> {
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

        let data = serde_json::to_string(&self)?;
        std::fs::write(self.path.clone(), data)?;

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        if self.test_mode {
            return Ok(());
        }
        
        let data = &read_to_string(self.path.clone())?;

        let db: Database = serde_json::from_str(data)?;
        self.projects = db.projects.clone();

        Ok(())
    }

    pub fn set_path(&mut self, path: String) -> Result<()> {
        self.path = format!("{}/database.json", path);
        self.load()?;

        Ok(())
    }

    pub fn set_test_mode(&mut self, test_mode: bool) {
        self.test_mode = test_mode;
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use super::*;

    #[test]
    fn test_database() {
        let dir = TempDir::new("test_database").expect("Failed to create temp dir");

        let mut db = Database::new();
        let _ = db.set_path(dir.path().to_str().unwrap().to_string());
        let project = GitProject::new("test");
        db.add_project(project.clone())
            .expect("Failed to add project");
        let mut db2 = Database::new();
        db2.set_path(dir.path().to_str().unwrap().to_string())
            .expect("Failed to set path and load database");

        assert_eq!(db.get_projects(), db2.get_projects());
    }
}
