use std::fmt;

#[derive(Debug)]
pub enum MlError {
    InvalidInput(String),
    Io(String),
    Serialization(String),
    Provider(String),
    ModelNotFound(String),
    DimensionMismatch { expected: usize, actual: usize },
}

impl fmt::Display for MlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "invalid ML input: {message}"),
            Self::Io(message) => write!(f, "ML I/O error: {message}"),
            Self::Serialization(message) => write!(f, "ML serialization error: {message}"),
            Self::Provider(message) => write!(f, "ML provider error: {message}"),
            Self::ModelNotFound(name) => write!(f, "ML model '{name}' is not registered"),
            Self::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "vector dimension mismatch: expected {expected}, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for MlError {}

pub type MlResult<T> = Result<T, MlError>;
