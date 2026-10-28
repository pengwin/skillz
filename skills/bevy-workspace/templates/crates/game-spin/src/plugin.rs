//! Registers the spin feature with the application.

use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};
use game_core::GameSet;

use crate::infrastructure::SpinSet;
use crate::messaging::FullTurn;
use crate::presentation::apply_spin;
use crate::simulation::advance_spin;

/// Registers the spin systems and messages.
pub struct SpinPlugin;

impl Plugin for SpinPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FullTurn>().add_systems(
            Update,
            (
                advance_spin
                    .in_set(GameSet::Simulation)
                    .in_set(SpinSet::Advance),
                apply_spin.in_set(GameSet::Presentation),
            ),
        );
    }
}
