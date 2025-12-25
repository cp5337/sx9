//! Git Lineage Module
//!
//! Semantic tracking of behavioral declarations across Git history.
//!
//! CLSGS Annex A.4 compliant: Git is treated as a semantic history carrier.
//! N-V-N-N annotations function as semantic lineage markers that must persist
//! or be explicitly revised across commits, branches, rebases, and cherry-picks.

mod tracker;
mod types;

pub use tracker::*;
pub use types::*;
