use bevy::{prelude::{Component, Vec3}, time::Timer};

#[derive(Default, Component, Clone, Debug)]
pub struct Platform {
    pub drop_timer: Timer,
    pub is_stepped_on: bool,
    pub initial_pos: Vec3,
    pub restart_timer: Timer,
}

