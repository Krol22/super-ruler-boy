use bevy::prelude::Component;

use super::ground_detector::WithPrevious;

#[derive(Debug, Default, Component)]
pub struct Interaction {
    pub is_overlapping: bool,
}
