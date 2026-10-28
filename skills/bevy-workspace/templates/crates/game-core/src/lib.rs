//! Shared contracts between feature crates: types only, no systems and no logic.
//!
//! A type moves here once two sibling features need it. Group contracts by
//! domain, one concept per file.

mod game_set;

pub use game_set::GameSet;
