use serde_json;
use std::fs;
use std::io::Write;

use crate::study_cycle::StudyCycle;

pub fn read_json_database_file() -> StudyCycle {
    create_database_file_if_not_exists();

    let content = fs::read_to_string("./database.json").expect("failed to read database file");

    let study_cycle: StudyCycle =
        serde_json::from_str(&content).expect("failed to parse database content");

    study_cycle
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
