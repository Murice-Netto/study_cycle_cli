use serde_json;
use std::fs;

use crate::study_cycle::StudyCycle;

pub fn read_json_database_file() -> StudyCycle {
    let content = fs::read_to_string("./database.json").expect("failed to read database file");
    let study_cycle: StudyCycle =
        serde_json::from_str(&content).expect("failed to parse database content");
    study_cycle
}

pub fn udpate_json_database(data: StudyCycle) {
    let json_data = serde_json::to_string_pretty::<StudyCycle>(&data)
        .expect("failed to parse new data into json string");
    fs::write("./database.json", json_data).expect("failed to write into database file");
}
