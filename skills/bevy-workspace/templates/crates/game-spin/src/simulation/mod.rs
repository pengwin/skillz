//! Game rules of the spin feature; runs headless.

mod advance;
#[cfg(test)]
mod advance_tests;

pub use advance::advance_spin;
