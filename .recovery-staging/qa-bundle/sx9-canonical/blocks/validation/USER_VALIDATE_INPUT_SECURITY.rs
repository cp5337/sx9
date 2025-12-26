// USER_VALIDATE_INPUT_SECURITY
// user validate input security

/// Canonical Pattern Block (read-only)
/// Constraints: deterministic, no IO, no logging, no config, single responsibility.

pub const MAX_ALLOWED: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedInput(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    Empty,
    TooLong,
    ContainsNull,
}

impl ValidatedInput {
    pub fn new(s: &str) -> Self { Self(s.to_string()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

pub fn validate_input(input: &str) -> Result<ValidatedInput, ValidationError> {
    if input.is_empty() { return Err(ValidationError::Empty); }
    if input.len() > MAX_ALLOWED { return Err(ValidationError::TooLong); }
    if input.as_bytes().iter().any(|b| *b == 0) { return Err(ValidationError::ContainsNull); }
    Ok(ValidatedInput::new(input))
}
