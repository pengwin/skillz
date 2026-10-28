use bevy::prelude::SystemSet;

/// Ordering handle for other features: schedule after `Advance` to see the new angle.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpinSet {
    /// Spin angles are advanced.
    Advance,
}
