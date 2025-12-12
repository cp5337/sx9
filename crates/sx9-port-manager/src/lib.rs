//! CTAS-7 Real Port Manager Library
//!
//! Standalone port manager with major port blocks, mirror blocks, and deception settings.

pub mod handlers;
pub mod port_manager;
pub mod types;

pub use port_manager::PortManager;
pub use types::*;
