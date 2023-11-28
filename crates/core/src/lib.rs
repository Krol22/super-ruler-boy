use bevy::{prelude::{Plugin, App, KeyCode}, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_tweening::TweeningPlugin;
use particle::ParticlePlugin;
use camera::CameraPlugin;
use animation::AnimationPlugin;

use self::{physics::PhysicsPlugin, render::RenderPlugin, mouse::MousePlugin};

pub mod animation;
pub mod particle;
pub mod camera;
pub mod physics;
pub mod render;
pub mod mouse;

#[derive(Debug, Default)]
pub struct CorePlugin {}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Grave)),
            )
            .add_plugins(TweeningPlugin)
            .add_plugins(ShapePlugin)
            .add_plugins(AnimationPlugin {})
            .add_plugins(ParticlePlugin {})
            .add_plugins(CameraPlugin {})
            .add_plugins(RenderPlugin {})
            .add_plugins(MousePlugin {})
            .add_plugins(PhysicsPlugin {});
    }
}
