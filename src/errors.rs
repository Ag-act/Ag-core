#[derive(thiserror::Error, uniffi::Error, Debug)]
pub enum AgError {
    #[error("Failed to write config: {0}")]
    ConfigReadError(String),
    #[error("Failed to load config: {0}")]
    ConfigWriteError(String),
    #[error("Failed to load home data: {0}")]
    HomeReadError(String),
    #[error("Failed to write home data: {0}")]
    HomeWriteError(String),
}