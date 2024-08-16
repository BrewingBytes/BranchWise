// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod errors;
pub mod git;

use git::project_folder::open_git_project;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_git_project])
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
