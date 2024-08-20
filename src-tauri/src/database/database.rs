// This file contains the database struct and its implementation
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use tauri::api::path::app_data_dir;

use crate::git::git_project::GitProject;

lazy_static! {
    pub static ref DATABASE: Database = Database::load(&tauri::Config::default()).unwrap_or_else(|_| Database::new());
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

    fn add_project<'b>(&mut self, project: GitProject) {
        self.projects.push(project);
    }

    fn get_projects(&self) -> Vec<GitProject> {
        self.projects.clone()
    }

    fn save(&self, cfg: &tauri::Config) {
        let data = serde_json::to_string(&self).unwrap();
        let path = app_data_dir(cfg).unwrap().join("database.json");
        std::fs::write(path, data).unwrap();
    }

    fn load(cfg: &tauri::Config) -> Result<Database> {
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
