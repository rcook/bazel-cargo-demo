use thiserror::Error;

#[derive(Debug, Error)]
pub enum WidgetParseError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum DataAccessError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
