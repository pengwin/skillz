use bevy::prelude::{Entity, Message};

/// Written when an entity completes a full turn.
#[derive(Message, Debug, Clone, Copy)]
pub struct FullTurn {
    /// The entity which completed the turn.
    pub entity: Entity,
}
