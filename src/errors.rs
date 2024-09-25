use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("UI Error: {0}")]
    UIError(#[from] eframe::Error),
}

unsafe impl Send for AppError {}
unsafe impl Sync for AppError {}

pub fn to_anyhow(error: eframe::Error) -> anyhow::Error {
    anyhow::Error::from(AppError::from(error))
}