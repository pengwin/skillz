use bevy::prelude::{Scene, Transform, bsn};

use crate::data::{Spin, SpinSpeed};

/// A spinning entity. Compose it into other scenes or spawn it with `spinner.spawn()`.
#[must_use]
pub fn spinner() -> impl Scene {
    bsn! {
        Spin
        SpinSpeed(1.0)
        Transform
    }
}
