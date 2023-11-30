use bevy::{prelude::Component, reflect::Reflect};
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

#[derive(InspectorOptions, Reflect, Debug, Default, Component)]
#[reflect(InspectorOptions)]
pub struct Interaction {
    pub is_overlapping: bool,
    pub disabled: bool,
}
