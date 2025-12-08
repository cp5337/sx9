use thiserror::Error;

#[derive(Error, Debug)]
pub enum XSDError {
    #[error("Genetic meta-programming error: {0}")]
    GeneticError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Bincode error: {0}")]
    BincodeError(#[from] bincode::Error),

    #[error("Sled database error: {0}")]
    SledError(#[from] sled::Error),

    #[error("Hex decode error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, XSDError>;