use bevy::prelude::SystemSet;

/// Phases of a frame. Features attach systems to a phase; the application
/// bootstrap orders the phases.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    /// Reads controls and writes intents.
    Input,
    /// Applies game rules to simulation state.
    Simulation,
    /// Updates what the player sees from simulation state.
    Presentation,
}
