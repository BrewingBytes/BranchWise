// This file contains the database struct and its implementation
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::sync::Mutex;
use tauri::api::path::app_data_dir;

use crate::git::git_project::GitProject;

lazy_static! {
    pub static ref DATABASE: Mutex<Database> =
        Mutex::new(Database::load(&tauri::Config::default()).unwrap_or_else(|_| Database::new()));
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
    projects: Vec<GitProject>,
}

impl Database {
    fn new() -> Self {
        Database {
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

    pub fn save(&self, cfg: &tauri::Config) -> Result<()> {
        let data = serde_json::to_string(&self)?;
        let path = app_data_dir(cfg).unwrap().join("database.json");
        std::fs::write(path, data)?;

        Ok(())
    }

    pub fn load(cfg: &tauri::Config) -> Result<Database> {
        let path = app_data_dir(cfg).unwrap().join("database.json");
        let data = &read_to_string(path)?;

        Ok(serde_json::from_str(data)?)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_database() {
        let mut db = Database::new();
        let project = GitProject::new("test");
        db.add_project(project.clone());
        db.save(&tauri::Config::default());
        let db2 = Database::load(&tauri::Config::default()).expect("Failed to load database");

        assert_eq!(db.get_projects(), db2.get_projects());
    }
}
