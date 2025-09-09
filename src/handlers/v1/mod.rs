use crate::error::AppError;
use crate::handlers::Handler;
use crate::storage::Storage;

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
}
