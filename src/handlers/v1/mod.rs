use crate::error::AppError;
use crate::handlers::Handler;
use crate::storage::Storage;
use crate::study_cycle::Subject;

pub struct V1Handler {
    storage: Box<dyn Storage>,
}

impl V1Handler {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        V1Handler { storage }
    }
}

impl Handler for V1Handler {
    fn study_cycle_by_name(&self, name: &str) -> Result<(), AppError> {
        if let Some(subject) = self.storage.get_subject_by_name(name)? {
            if subject.studied_hours == subject.max_study_hours {
                return Err(AppError::Logic(format!(
                    "Subject '{}' is locked. (either reset the cycle or finisht other subjects)",
                    name
                )));
            }
            let mut cloned_subjcect = subject.clone();
            cloned_subjcect.studied_hours += 1;
            self.storage.update_subject(&cloned_subjcect)?;
            Ok(())
        } else {
            Err(AppError::Logic(format!(
                "Subject '{}' was not found.",
                name
            )))
        }
    }

    fn reset_cycle(&self) -> Result<(), AppError> {
        let mut db = self.storage.read_cycle()?;
        let uncompleted_subjects: Vec<&Subject> = db
            .subjects
            .iter()
            .filter(|s| s.studied_hours != s.max_study_hours)
            .collect();
        let still_need_to_study = !uncompleted_subjects.is_empty();
        if still_need_to_study {
            let mut error_message =
                "Failed to restart cycle. The following subjects need to be finished:".to_string();
            for subject in uncompleted_subjects {
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
        self.storage.write_cycle(&db)?;
        Ok(())
    }
}
