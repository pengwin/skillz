//! Application bootstrap.

/// Run the application.
pub fn run() {
    eprintln!("Application ready.");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn application_bootstrap_completes_without_panicking() {
        run();
    }
}
