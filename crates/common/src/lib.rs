use bevy::prelude::{Plugin, App};

pub mod bundles;
pub mod components;
pub mod assets;

#[derive(Debug, Default)]
pub struct CommonPlugin {}

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
    }
}
