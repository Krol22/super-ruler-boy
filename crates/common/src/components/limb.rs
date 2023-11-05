use bevy::prelude::Component;
use bevy_inspector_egui::InspectorOptions;

#[derive(Debug)]
pub enum LimbType {
    Body,
    Hands,
    Legs,
    Extension,
}

#[derive(Component, InspectorOptions, Debug)]
pub struct Limb {
    pub limb_type: LimbType,
}

impl Limb {
    pub fn new(limb_type: LimbType) -> Limb {
        Limb {
            limb_type,
        }
    }
}
