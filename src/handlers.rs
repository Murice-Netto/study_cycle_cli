use std::fs;

use crate::model::{self, read_json_database_file, udpate_json_database};
use crate::study_cycle::{StudyCycle, Subject};

pub fn study_subject(name: String) {
    let mut db = read_json_database_file();

    let subject = db.subjects.iter_mut().find(|s| s.name == name);

    match subject {
        Some(subject) => {
            if subject.studied_hours >= subject.max_study_hours {
                println!(
                    "locked subject: {}/{}h studied already.",
                    subject.studied_hours, subject.max_study_hours
                );
                return;
            }

            subject.studied_hours += 1;

            udpate_json_database(db);

            println!("studied!")
        }
        None => println!("subject '{}' not found.", name),
    }
}

pub fn view_study_cycle(all: bool) {
    let db = read_json_database_file();

    match all {
        true => {
            for subject in db.subjects {
                println!(
                    "{} - {}/{}h",
                    subject.name, subject.studied_hours, subject.max_study_hours
                )
            }
        }
        false => {
            for subject in db
                .subjects
                .iter()
                .filter(|s| s.studied_hours < s.max_study_hours)
            {
                println!(
                    "{} - {}/{}h",
                    subject.name, subject.studied_hours, subject.max_study_hours
                )
            }
        }
    }
}

pub fn reset_cycle() {
    let mut db = read_json_database_file();
    let mut still_need_study = Vec::<&Subject>::new();

    for subject in db.subjects.iter() {
        if subject.studied_hours != subject.max_study_hours {
            still_need_study.push(subject);
        }
    }

    if still_need_study.iter().len() > 0 {
        println!("failed to reset cycle. still need to study these subjects:");

        for subject in still_need_study {
            println!(
                "{} - {}/{}h",
                subject.name, subject.studied_hours, subject.max_study_hours
            )
        }

        return;
    }

    for subject in db.subjects.iter_mut() {
        subject.studied_hours = 0
    }

    udpate_json_database(db);
}

pub fn seed_database(path: String) {
    model::create_database_file_if_not_exists();

    let content = fs::read_to_string(path).expect("failed to get file data to seed database");

    let db: StudyCycle = serde_json::from_str(&content).expect("failed to parse seed content");

    udpate_json_database(db);
}
