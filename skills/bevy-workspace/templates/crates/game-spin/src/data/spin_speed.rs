use bevy::prelude::Component;

/// Angular speed in radians per second.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SpinSpeed(pub f32);
