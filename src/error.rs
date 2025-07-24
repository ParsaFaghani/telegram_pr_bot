use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("خطای HTTP: {0}")]
    Http(#[from] reqwest::Error),

    #[error("خطای JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("خطای دیگر: {0}")]
    Other(String),
}
