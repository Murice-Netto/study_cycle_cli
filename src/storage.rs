use crate::error::AppError;
use crate::study_cycle::StudyCycle;

pub trait Storage {
    fn read_cycle(&self) -> Result<StudyCycle, AppError>;
    fn write_cycle(&self, cycle: &StudyCycle) -> Result<(), AppError>;
}
