// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod database;
pub mod errors;
pub mod git;

use std::fs;

use database::storage::DATABASE;
use errors::git_error::GitErrorProject;
use git::project_folder::{
    get_database_projects, open_git_project, remove_database_project, set_current_project,
};
use tauri::{AppHandle, Manager};

async fn setup(app: AppHandle) {
    fs::create_dir_all(app.path_resolver().app_data_dir().unwrap())
        .expect("Failed to create app data directory");

    let _ = DATABASE.lock().unwrap().set_path(
        app.path_resolver()
            .app_data_dir()
            .unwrap()
            .display()
            .to_string(),
    );
}

async fn event_loop(app: AppHandle) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        let mutex = DATABASE.lock().unwrap();
        if let Some(mut project) = mutex.get_current_project() {
            drop(mutex);
            match project.update() {
                Ok(_) => {
                    app.emit_all("project_update", &project).unwrap();

                    let _ = DATABASE.lock().unwrap().update_project(project.clone());
                }
                Err(e) => {
                    app.emit_all("project_update_error", GitErrorProject::new(e, project.clone()))
                        .unwrap();

                    let _ = DATABASE.lock().unwrap().update_project(project.clone());
                }
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(setup(app.handle()));
            tauri::async_runtime::spawn(event_loop(app.handle()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_git_project,
            get_database_projects,
            remove_database_project,
            set_current_project
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
