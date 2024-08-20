// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod database;
pub mod errors;
pub mod git;

use std::fs;

use git::project_folder::{open_git_project, save_database, get_database_projects};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            fs::create_dir_all(app.handle().path_resolver().app_data_dir().unwrap())
                .expect("Failed to create app data directory");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_git_project, save_database, get_database_projects])
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
