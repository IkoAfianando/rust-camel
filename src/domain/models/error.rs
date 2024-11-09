use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Processor error: {0}")]
    ProcessorError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
