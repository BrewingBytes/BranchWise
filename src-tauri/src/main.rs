// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod database;
pub mod errors;
pub mod git;
pub mod packs;
pub mod log;

use ::log::debug;
use std::fs;

use database::storage::DATABASE;
use errors::git_error::GitErrorProject;
use git::project_folder::{
    get_commit_history, get_database_projects, open_git_project, remove_database_project,
    set_current_project,
};
use tauri::{AppHandle, Emitter, Manager};

/// Initializes the application logger and sets up the app data directory and database path.
///
/// Creates the application data directory if it does not exist and configures the database to use this directory for storage. The logger is initialized in debug builds.
///
/// # Arguments
///
/// * `app` - The Tauri application handle used to access application-specific paths.
///
/// # Panics
///
/// Panics if the application data directory cannot be created.
async fn setup(app: AppHandle) {
    // Init logger
    #[cfg(debug_assertions)]
    let _ = log::init();


    debug!("Started app setup");
    // Create the app data directory if it doesn't exist
    fs::create_dir_all(app.path().app_data_dir().unwrap())
        .expect("Failed to create app data directory");

    // Initialize the database with the app data directory path
    _ = DATABASE
        .lock()
        .unwrap()
        .set_path(app.path().app_data_dir().unwrap().display().to_string());
}

/// Periodically updates the current Git project and emits update events to the frontend.
///
/// Every 5 seconds, checks for a current project in the database. If one exists, attempts to update it:
/// - On success, emits a `"project_update"` event with the updated project.
/// - On failure, emits a `"project_update_error"` event with error details and the project.
/// In both cases, the project is updated in the database after each attempt.
///
/// This function runs indefinitely as part of the application's background event loop.
async fn event_loop(app: AppHandle) {
    // Create a new interval that ticks every 5 seconds to update the current project
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    loop {
        interval.tick().await;

        // Lock the database to get the current project
        let mutex = DATABASE.lock().unwrap();
        if let Some(mut project) = mutex.get_current_project() {
            drop(mutex);

            // Update the current project
            match project.update() {
                Ok(_) => {
                    debug!("Send project_update event for project {}", &project.get_directory());

                    // Emit the project update event and update the project in the database
                    app.emit("project_update", &project).unwrap();

                    _ = DATABASE.lock().unwrap().update_project(project.clone());
                }
                Err(e) => {
                    debug!("Project_update event failed");

                    // Emit the project update error event and update the project in the database
                    app.emit(
                        "project_update_error",
                        GitErrorProject::new(e, project.clone()),
                    )
                    .unwrap();

                    _ = DATABASE.lock().unwrap().update_project(project.clone());
                }
            }
        }
    }
}

/**
 * Create a new tauri application with the following plugins:
 * - tauri-plugin-process
 * - tauri-plugin-dialog
 * - tauri-plugin-updater
 * - tauri-plugin-shell
 *
 * Setup the application by creating the app data directory and setting the database path.
 * Also, start the event loop to update the current project every 5 seconds.
 */
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(setup(app.handle().clone()));
            tauri::async_runtime::spawn(event_loop(app.handle().clone()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_git_project,
            get_database_projects,
            remove_database_project,
            set_current_project,
            get_commit_history,
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
