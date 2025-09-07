use crate::error::AppError;
use crate::study_cycle::{StudyCycle, Subject};

pub trait Storage {
    fn read_cycle(&self) -> Result<StudyCycle, AppError>;
    fn write_cycle(&self, cycle: &StudyCycle) -> Result<(), AppError>;
    fn get_subject_by_name(&self, name: &str) -> Result<Option<Subject>, AppError>;
    fn update_subject(&self, subject_to_update: &Subject) -> Result<(), AppError>;
}
