use bevy::{prelude::Component, reflect::Reflect};
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct Player {
    pub stretch: f32,
    pub stretch_dir: isize,
    pub grabbed_ceiling: bool,
    // #TODO ENUM
    pub has_x_collision: isize,
}
