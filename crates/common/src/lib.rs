use bevy::prelude::{Plugin, App};
use components::player::Player;

pub mod bundles;
pub mod components;
pub mod assets;

#[derive(Debug, Default)]
pub struct CommonPlugin {}

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>();
    }
}
