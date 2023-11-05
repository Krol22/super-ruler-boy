use bevy::prelude::{Plugin, App};
use bevy_rapier2d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};

const PPM: f32 = 10.0;

#[derive(Debug, Default)]
pub struct PhysicsPlugin {}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM));
    }
}
