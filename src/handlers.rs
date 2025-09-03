use std::fs;

use crate::error::AppError;
use crate::model::{self, read_json_database_file, udpate_json_database};
use crate::study_cycle::{StudyCycle, Subject};
use crate::utils;

pub fn study_subject(name: String) -> Result<(), AppError> {
    let mut db = read_json_database_file();

    if let Some(subject) = db.subjects.iter_mut().find(|s| s.name == name) {
        if subject.studied_hours >= subject.max_study_hours {
            return Err(AppError::Logic(format!(
                "Blocked subject: {}/{}h studied.",
                subject.studied_hours, subject.max_study_hours
            )));
        }

        subject.studied_hours += 1;

        udpate_json_database(db);

        println!("Studied!");
    } else {
        return Err(AppError::Logic(format!(
            "Subject '{}' was not found.",
            name
        )));
    }

    Ok(())
}

pub fn view_study_cycle(all: bool) -> Result<(), AppError> {
    let db = read_json_database_file();

    if db.subjects.is_empty() {
        println!("No subjects were found.");
        return Ok(());
    }

    if all {
        utils::display_table_with_progress_bar(db.subjects);
    } else {
        let filtered_subjects: Vec<Subject> = db
            .subjects
            .iter()
            .filter(|s| s.studied_hours < s.max_study_hours)
            .cloned()
            .collect();
        utils::display_table_with_progress_bar(filtered_subjects);
    }

    Ok(())
}

pub fn reset_cycle() -> Result<(), AppError> {
    let mut db = read_json_database_file();

    let still_need_study: Vec<&Subject> = db
        .subjects
        .iter()
        .filter(|s| s.studied_hours != s.max_study_hours)
        .collect();

    if !still_need_study.is_empty() {
        let mut error_message =
            "Failed to reset cycle. The following subjects need to be finished:".to_string();
        for subject in still_need_study {
            error_message.push_str(&format!(
                "\n - {}({}/{}h)",
                subject.name, subject.studied_hours, subject.max_study_hours
            ));
        }
        return Err(AppError::Logic(error_message));
    }

    for subject in db.subjects.iter_mut() {
        subject.studied_hours = 0;
    }

    println!("Cycle reseted!");

    udpate_json_database(db);

    Ok(())
}

pub fn seed_database(path: String) {
    model::create_database_file_if_not_exists();

    let content = fs::read_to_string(path).expect("failed to get file data to seed database");

    let db: StudyCycle = serde_json::from_str(&content).expect("failed to parse seed content");

    udpate_json_database(db);
}
