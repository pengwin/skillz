//! Application bootstrap: what runs when the program starts.

mod run;
#[cfg(test)]
mod run_tests;

pub use run::run;
