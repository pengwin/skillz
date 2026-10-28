use core::f32::consts::TAU;

use bevy::prelude::{Entity, MessageWriter, Query, Res, Time};

use crate::data::{Spin, SpinSpeed};
use crate::messaging::FullTurn;

/// Advances every spin by its speed and reports completed turns. Touches simulation state only.
#[expect(clippy::needless_pass_by_value, reason = "Bevy system parameters")]
pub fn advance_spin(
    time: Res<'_, Time>,
    mut spins: Query<'_, '_, (Entity, &SpinSpeed, &mut Spin)>,
    mut turns: MessageWriter<'_, FullTurn>,
) {
    for (entity, speed, mut spin) in &mut spins {
        let before = (spin.angle / TAU).floor();
        spin.angle = speed.0.mul_add(time.delta_secs(), spin.angle);
        if (spin.angle / TAU).floor() > before {
            turns.write(FullTurn { entity });
        }
    }
}
