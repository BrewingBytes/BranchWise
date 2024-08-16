// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod errors;

use std::fs;

use errors::GitError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! (test)", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn open_git_project(directory: &str) -> Result<(), errors::GitError> {
    match fs::read_dir(directory) {
        Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            println!("Checking folder: {:?}", path);
                            if path.ends_with(".git") {
                                return Ok(());
                            }
                        }
                    }
                    Err(_) => return Err(GitError::CannotOpenFolder),
                }
            }

            Err(GitError::NoGitFolder)
        },
        Err(_) => Err(GitError::CannotOpenFolder),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_git_project() {
        assert_eq!(open_git_project(".."), Ok(()));
    }

    #[test]
    fn test_open_git_project_error() {
        assert_eq!(open_git_project("nonexistent"), Err(GitError::CannotOpenFolder));
    }

    #[test]
    fn test_open_git_project_no_git() {
        assert_eq!(open_git_project("."), Err(GitError::NoGitFolder));
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World! You've been greeted from Rust! (test)");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, ! You've been greeted from Rust! (test)");
    }

    #[test]
    #[should_panic]
    fn test_application_main() {
        main();
    }
}
