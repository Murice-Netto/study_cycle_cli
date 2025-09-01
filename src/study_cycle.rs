use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StudyCycle {
    pub subjects: Vec<Subject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub name: String,
    #[serde(rename = "maxStudyHours")]
    pub max_study_hours: u8,
    #[serde(rename = "studiedHours")]
    pub studied_hours: u8,
}
