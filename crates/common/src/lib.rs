use bevy::prelude::{Plugin, App};
use components::{player::Player, checkpoint::Checkpoint, interaction::Interaction};
use events::{PinUiUpdated};

pub mod bundles;
pub mod components;
pub mod assets;
pub mod resources;
pub mod events;

#[derive(Debug, Default)]
pub struct CommonPlugin {}

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Checkpoint>()
            .register_type::<Player>()
            .register_type::<Interaction>()
            .add_event::<PinUiUpdated>();
    }
}
