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
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    path: String,
    projects: Vec<GitProject>,
}

impl Database {
    fn new() -> Self {
        Database {
            path: String::new(),
            projects: Vec::new(),
        }
    }

    pub fn add_project(&mut self, project: GitProject) {
        self.projects.push(project);
    }

    pub fn remove_project(&mut self, project: GitProject) {
        self.projects.retain(|p| p != &project);
    }

    pub fn get_projects(&self) -> Vec<GitProject> {
        self.projects.clone()
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string(&self)?;
        std::fs::write(self.path.clone(), data)?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let data = &read_to_string(self.path.clone())?;

        let db: Database = serde_json::from_str(data)?;
        self.projects = db.projects.clone();

        Ok(())
    }

    pub fn set_path(&mut self, path: String) {
        self.path = format!("{}/database.json", path);

        dbg!(&self.path);
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
        db.set_path(dir.path().to_str().unwrap().to_string());
        let project = GitProject::new("test");
        db.add_project(project.clone());
        db.save().expect("Failed to save database");
        let mut db2 = Database::new();
        db2.set_path(dir.path().to_str().unwrap().to_string());
        db2.load().expect("Failed to load database");


        assert_eq!(db.get_projects(), db2.get_projects());
    }
}
