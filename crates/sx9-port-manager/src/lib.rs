#![allow(clippy::items_after_statements)]
#![allow(clippy::format_push_string)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unused_async)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unused_self)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
//! CTAS-7 Real Port Manager Library
//!
//! Standalone port manager with major port blocks, mirror blocks, and deception settings.

pub mod handlers;
pub mod port_manager;
pub mod types;

pub use port_manager::PortManager;
pub use types::*;
