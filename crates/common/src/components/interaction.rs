use bevy::prelude::Component;

#[derive(Debug, Default, Component)]
pub struct Interaction {
    pub is_overlapping: bool,
    pub disabled: bool,
}
