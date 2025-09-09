pub mod v1;

use crate::error::AppError;

pub trait Handler {
    fn study_cycle_by_name(&self, name: &str) -> Result<(), AppError>;
    fn show_cycle(&self, all: bool) -> Result<(), AppError>;
    fn reset_cycle(&self) -> Result<(), AppError>;
}
