use bevy::prelude::Component;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Default)]
pub struct Player {
    pub stretch: f32,
}
