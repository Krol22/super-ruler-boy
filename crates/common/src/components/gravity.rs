use bevy::prelude::Component;

#[derive(Debug, Default, Component)]
pub struct GravityDir {
    pub dir: f32,
    pub slow_down: f32,
}

