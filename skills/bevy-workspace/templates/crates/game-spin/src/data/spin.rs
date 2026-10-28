use bevy::prelude::Component;

/// Current rotation around the Z axis, in radians.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Spin {
    /// The accumulated angle.
    pub angle: f32,
}
