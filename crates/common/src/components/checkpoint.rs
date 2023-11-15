use bevy::{prelude::Component, reflect::Reflect};
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct Checkpoint {
    pub is_active: bool,
}
