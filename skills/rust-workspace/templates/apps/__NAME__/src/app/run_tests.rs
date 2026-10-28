use rstest::rstest;

use super::run::run;

#[rstest]
fn application_bootstrap_completes_without_panicking() {
    run();
}
