use bevy::prelude::{Component, Vec2, Color};

use super::ground_detector::WithPrevious;

#[derive(Default, Clone, Debug, PartialEq)]
pub enum PinState {
    #[default] Idle,
    Picked,
}

#[derive(Default, Component, Debug)]
pub struct Pin {
    pub state: WithPrevious<PinState>,
    pub initial_position: Vec2,
    pub picked: bool,

    pub position: Vec2,
    pub color: Color,
}
