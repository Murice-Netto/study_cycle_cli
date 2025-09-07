use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudyCycle {
    pub subjects: Vec<Subject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub name: String,
    pub max_study_hours: u8,
    pub studied_hours: u8,
}
