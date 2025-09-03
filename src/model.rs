use serde_json;
use std::fs;
use std::io::Write;

use crate::error::AppError;
use crate::study_cycle::StudyCycle;

const DB_PATH: &str = "./database.json";

pub fn read_json_database_file() -> Result<StudyCycle, AppError> {
    match fs::read_to_string(DB_PATH) {
        Ok(content) => {
            let study_cycle: StudyCycle = serde_json::from_str(&content)?;

            Ok(study_cycle)
        }

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default_cycle = StudyCycle { subjects: vec![] };

            udpate_json_database(&default_cycle)?;

            Ok(default_cycle)
        }

        Err(e) => Err(e.into()),
    }
}

pub fn udpate_json_database(data: StudyCycle) {
    create_database_file_if_not_exists();

    let json_data = serde_json::to_string_pretty::<StudyCycle>(&data)
        .expect("failed to parse new data into json string");
    fs::write("./database.json", json_data).expect("failed to write into database file");
}

pub fn create_database_file_if_not_exists() {
    if !fs::exists("./database.json").expect("could not check for file existence") {
        let content = r#"{"subjects": []}"#;

        let mut file =
            fs::File::create("./database.json").expect("failed to create database.json file");

        file.write_all(content.as_bytes())
            .expect("failed to write content in database.json file");
    }
}
