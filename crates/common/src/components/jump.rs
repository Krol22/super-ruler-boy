use bevy::{time::Timer, prelude::Component, reflect::Reflect};
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct Jump {
    pub can_jump: bool,
    pub jump_timer: Timer,
    pub is_jumping: bool,
}
