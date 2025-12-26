// SERVICE_ROTATE_TOKEN_SECURE
// service rotate token secure

/// Canonical Pattern Block (read-only)
/// Constraints: deterministic, no IO, no logging, no config, single responsibility.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nonce(pub [u8; 12]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RotateError {
    InvalidSeed,
}

/// NOTE: Pure deterministic rotation stub. In production, seed comes from a trusted entropy source.
pub fn rotate_token(seed: &[u8]) -> Result<Token, RotateError> {
    if seed.len() < 32 { return Err(RotateError::InvalidSeed); }
    let mut out = [0u8; 32];
    out.copy_from_slice(&seed[..32]);
    // Deterministic whitening (toy): XOR with reversed bytes.
    for i in 0..32 { out[i] ^= seed[31 - i]; }
    Ok(Token(out))
}
