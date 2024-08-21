// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod database;
pub mod errors;
pub mod git;

use std::fs;

use database::database::DATABASE;
use git::project_folder::{get_database_projects, open_git_project};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            fs::create_dir_all(app.handle().path_resolver().app_data_dir().unwrap())
                .expect("Failed to create app data directory");

            DATABASE.lock().unwrap().set_path(
                app.handle()
                    .path_resolver()
                    .app_data_dir()
                    .unwrap()
                    .display().to_string()
            ).expect("Failed to set database path and load it");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_git_project,
            get_database_projects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_application_main() {
        main();
    }
}
