use bevy::prelude::{Quat, Query, Transform};

use crate::data::Spin;

/// Mirrors the simulated angle onto the rendered transform.
pub fn apply_spin(mut spinning: Query<'_, '_, (&Spin, &mut Transform)>) {
    for (spin, mut transform) in &mut spinning {
        transform.rotation = Quat::from_rotation_z(spin.angle);
    }
}
