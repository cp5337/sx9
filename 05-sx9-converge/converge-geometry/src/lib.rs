//! CONVERGE Geometry Helper Crate
//!
//! RFC Compliance:
//! - RFC-93X2 Geometry Boundary Specification
//!
//! Purpose:
//! Provide geometry and kinematic primitives for CONVERGE.
//!
//! Restrictions:
//! - MUST NOT influence independence or selection
//! - MAY adjust confidence only

pub mod earth;
pub mod enu;
pub mod intercept;
