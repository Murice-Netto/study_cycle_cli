use serde_json;
use std::fs;

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

pub fn udpate_json_database(data: &StudyCycle) -> Result<(), AppError> {
    let json_data = serde_json::to_string_pretty(data)?;

    fs::write(DB_PATH, json_data)?;

    Ok(())
}
