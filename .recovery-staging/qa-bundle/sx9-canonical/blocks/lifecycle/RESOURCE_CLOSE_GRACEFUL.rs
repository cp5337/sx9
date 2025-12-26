// RESOURCE_CLOSE_GRACEFUL
// resource close graceful lifecycle

/// Canonical Pattern Block (read-only)
/// Constraints: deterministic, no IO, no logging, no config, single responsibility.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloseState {
    Open,
    Closing,
    Closed,
}

pub fn next_close_state(state: CloseState) -> CloseState {
    match state {
        CloseState::Open => CloseState::Closing,
        CloseState::Closing => CloseState::Closed,
        CloseState::Closed => CloseState::Closed,
    }
}
