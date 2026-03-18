#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("invalid confidence")]
    InvalidConfidence,
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Storage(#[from] rusqlite::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
    #[error(transparent)]
    TimeParse(#[from] chrono::ParseError),
    #[error("query error: {0}")]
    Query(String),
}
