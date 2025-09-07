use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::error::AppError;
use crate::storage::Storage;
use crate::study_cycle::{StudyCycle, Subject};

pub struct JsonStorage {
    path: PathBuf,
}

impl JsonStorage {
    pub fn new(path: impl AsRef<Path>) -> Self {
        JsonStorage {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Storage for JsonStorage {
    fn read_cycle(&self) -> Result<StudyCycle, AppError> {
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                let study_cycle: StudyCycle = serde_json::from_str(&content)?;
                Ok(study_cycle)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let default_cycle = StudyCycle { subjects: vec![] };
                self.write_cycle(&default_cycle)?;
                Ok(default_cycle)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn write_cycle(&self, cycle: &StudyCycle) -> Result<(), AppError> {
        let json_data = serde_json::to_string_pretty(cycle)?;
        fs::write(&self.path, json_data)?;
        Ok(())
    }

    fn get_subject_by_name(&self, name: &str) -> Result<Option<Subject>, AppError> {
        let cycle = self.read_cycle()?;
        let found_subject = cycle.subjects.iter().find(|s| s.name == name);
        Ok(found_subject.cloned())
    }

    fn update_subject(&self, subject_to_update: &Subject) -> Result<(), AppError> {
        let mut cycle = self.read_cycle()?;
        let position = cycle
            .subjects
            .iter()
            .position(|s| s.name == subject_to_update.name);
        if let Some(pos) = position {
            cycle.subjects[pos] = subject_to_update.clone();
        } else {
            return Err(AppError::Logic(format!(
                "Failed to update: '{}' was not found.",
                subject_to_update.name
            )));
        }
        self.write_cycle(&cycle)
    }
}
