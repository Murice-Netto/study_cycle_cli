use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::error::AppError;
use crate::study_cycle::{StudyCycle, Subject};

pub fn get_biggest_string_len(strings: &[&str]) -> usize {
    strings.iter().map(|s| s.len()).max().unwrap_or(0)
}

pub fn display_table(subjects: &[Subject]) {
    let name_col_len = subjects
        .iter()
        .map(|s| s.name.len())
        .max()
        .unwrap_or(0)
        .max("NAME".len());
    let studied_col_len = "STUDIED HOURS".len();
    let max_col_len = "MAX STUDY HOURS".len();
    println!(
        "{0: <name_col_len$} | {1: <studied_col_len$} | {2: <max_col_len$}",
        "NAME", "STUDIED HOURS", "MAX STUDY HOURS"
    );
    for subject in subjects {
        println!(
            "{0: <name_col_len$} | {1: <studied_col_len$} | {2: <max_col_len$}",
            subject.name, subject.studied_hours, subject.max_study_hours
        );
    }
}

pub fn get_json_database_path() -> Result<PathBuf, AppError> {
    const APP_FOLDER: &str = "StudyCycleCLI";
    const FILE_NAME: &str = "database.json";
    if let Some(home_dir) = dirs::home_dir() {
        match std::env::consts::OS {
            "windows" => {
                let database_path = Path::new(&home_dir)
                    .join("AppData")
                    .join("Local")
                    .join(APP_FOLDER);
                let database_file_path = database_path.join(FILE_NAME);
                if !database_path.exists() {
                    fs::create_dir(&database_path)?;
                    let starter_db = StudyCycle {
                        subjects: vec![Subject {
                            name: "teste".to_string(),
                            studied_hours: 0,
                            max_study_hours: 10,
                        }],
                    };
                    let content = serde_json::to_string_pretty::<StudyCycle>(&starter_db)?;
                    File::options()
                        .read(true)
                        .write(true)
                        .create_new(true)
                        .open(&database_file_path)?;
                    fs::write(&database_file_path, content)?;
                }
                Ok(database_file_path)
            }
            "linux" | "macos" => {
                let database_path = Path::new(&home_dir).join(".config").join(APP_FOLDER);
                let database_file_path = database_path.join(FILE_NAME);
                if !database_path.exists() {
                    fs::create_dir(&database_path)?;
                    let starter_db = StudyCycle {
                        subjects: vec![Subject {
                            name: "teste".to_string(),
                            studied_hours: 0,
                            max_study_hours: 10,
                        }],
                    };
                    let content = serde_json::to_string_pretty::<StudyCycle>(&starter_db)?;
                    File::options()
                        .read(true)
                        .write(true)
                        .create_new(true)
                        .open(&database_file_path)?;
                    fs::write(&database_file_path, content)?;
                }
                Ok(database_file_path)
            }
            _ => Err(AppError::Logic(format!(
                "{:?} OS is not supported.",
                home_dir
            ))),
        }
    } else {
        Err(AppError::Logic(
            "Failed to get user's home folder.".to_string(),
        ))
    }
}

pub fn display_table_with_progress_bar(subjects: &[Subject]) {
    if subjects.is_empty() {
        println!("No subjects were found.");
        return;
    }
    let name_width = subjects
        .iter()
        .map(|s| s.name.len())
        .max()
        .unwrap_or(0)
        .max("NAME".len());
    let progress_width = subjects
        .iter()
        .map(|s| s.max_study_hours as usize)
        .max()
        .unwrap_or(0)
        .max("PROGRESS BAR".len());
    let hours_width = subjects
        .iter()
        .map(|s| format!("{}/{}h", s.studied_hours, s.max_study_hours).len())
        .max()
        .unwrap_or(0)
        .max("HOURS".len());
    println!(
        "{0: <name_width$} | {1: <progress_width$} | {2: <hours_width$}",
        "NAME", "PROGRESS BAR", "HOURS"
    );
    for subject in subjects {
        let progress_bar =
            get_study_hours_progress_bar(subject.studied_hours, subject.max_study_hours);
        let hours_text = format!("{}/{}h", subject.studied_hours, subject.max_study_hours);
        println!(
            "{0: <name_width$} | {1: <progress_width$} | {2: <hours_width$}",
            subject.name, progress_bar, hours_text
        );
    }
}

pub fn get_study_hours_progress_bar(studied_hours: u8, max_study_hours: u8) -> String {
    const EMPTY: char = '□';
    const FULL: char = '■';
    let full_part: String = std::iter::repeat(FULL)
        .take(studied_hours as usize)
        .collect();
    let empty_parts: String = std::iter::repeat(EMPTY)
        .take((max_study_hours - studied_hours) as usize)
        .collect();
    format!("{}{}", full_part, empty_parts)
}
