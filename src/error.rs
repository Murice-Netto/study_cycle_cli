#[derive(Debug)]
pub enum AppError {
    Logic(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::Json(value)
    }
}
