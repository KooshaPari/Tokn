//! ParetoOptimal cost engine — pure pricing & routing logic.
//!
//! No I/O, no CLI, no external API calls. Just pure business logic.

pub mod cost;
pub mod format;
pub mod models;
pub mod pricing;
pub mod utils;

pub use cost::*;
pub use models::*;
pub use pricing::*;
