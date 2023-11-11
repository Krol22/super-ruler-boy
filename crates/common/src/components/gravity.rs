use bevy::prelude::Component;

#[derive(Debug, Default, Component)]
pub struct GravityDir {
    pub dir: isize,
}

