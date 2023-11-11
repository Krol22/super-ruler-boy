use bevy::prelude::{Component, Vec2};

#[derive(Debug, Component, Default)]
pub struct Acceleration {
    pub current: Vec2,
}

