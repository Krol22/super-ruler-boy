use bevy::prelude::{Component, Vec2};

#[derive(Debug, Component)]
pub struct Velocity {
    pub current: Vec2,
    pub max: Vec2,
    pub damping: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            current: Default::default(),
            max: Vec2::new(100.0, 200.0),
            damping: 0.0
        }
    }
}
