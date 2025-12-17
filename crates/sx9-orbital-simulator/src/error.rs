//! Error types for orbital mechanics calculations

use thiserror::Error;

/// Result type alias for orbital mechanics operations
pub type Result<T> = std::result::Result<T, OrbitalMechanicsError>;

/// Error types for orbital mechanics operations
#[derive(Error, Debug)]
pub enum OrbitalMechanicsError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Satellite not found: {0}")]
    SatelliteNotFound(String),

    #[error("Ground station not found: {0}")]
    GroundStationNotFound(String),

    #[error("Invalid orbital elements: {0}")]
    InvalidOrbitalElements(String),

    #[error("Propagation error: {0}")]
    PropagationError(String),

    #[error("Coordinate conversion error: {0}")]
    CoordinateError(String),

    #[error("Visibility calculation error: {0}")]
    VisibilityError(String),

    #[error("FSO analysis error: {0}")]
    FsoAnalysisError(String),

    #[error("Time error: {0}")]
    TimeError(String),

    #[error("Mathematical error: {0}")]
    MathematicalError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Date/time parsing error: {0}")]
    ChronoError(#[from] chrono::ParseError),
}

impl OrbitalMechanicsError {
    /// Create a configuration error
    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Create a propagation error
    pub fn propagation_error(msg: impl Into<String>) -> Self {
        Self::PropagationError(msg.into())
    }

    /// Create a mathematical error
    pub fn math_error(msg: impl Into<String>) -> Self {
        Self::MathematicalError(msg.into())
    }

    /// Create an invalid orbital elements error
    pub fn invalid_elements(msg: impl Into<String>) -> Self {
        Self::InvalidOrbitalElements(msg.into())
    }
}
