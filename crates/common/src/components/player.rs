use bevy::prelude::Component;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Default)]
pub struct Player {
    pub stretch: f32,
    pub stretch_dir: isize,
    // #TODO ENUM
    pub has_x_collision: isize,
}
