//! Example leaf feature: entities which spin around the Z axis.
//!
//! Copy this crate as the starting point of a new feature.

mod data;
mod infrastructure;
mod messaging;
mod plugin;
mod presentation;
mod simulation;

pub use data::{Spin, SpinSpeed};
pub use infrastructure::SpinSet;
pub use messaging::FullTurn;
pub use plugin::SpinPlugin;
pub use presentation::spinner;
