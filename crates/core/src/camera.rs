use std::time::Duration;
use bevy_parallax::*;

use bevy::{prelude::*, window::PrimaryWindow, core_pipeline::bloom::BloomSettings};
use bevy_tweening::{Animator, Tween, EaseFunction, lens::TransformPositionLens};
use kt_util::constants::{ASPECT_RATIO_X, ASPECT_RATIO_Y};

fn spawn_camera(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let tween = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_secs_f32(0.2),
        TransformPositionLens {
            start: Vec3::splat(0.01),
            end: Vec3::ONE,
        },
    );

    let camera = commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ParallaxCameraComponent::default(),
        BloomSettings {
            intensity: 0.1,
            ..default()
        },
        // Animator::new(tween)
    )).id();

    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.2, 0.1),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "sprites/background.png".to_string(),
                tile_size: Vec2::new(622.0, 317.0),
                cols: 1,
                rows: 1,
                scale: 1.2,
                z: -1.0,
                ..default()
            },
        ],
        camera,
    })
}

pub fn auto_scale_sys(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let window = window_query.get_single().unwrap();

    for mut transform in query.iter_mut() {
        let scale = (ASPECT_RATIO_Y / window.height()).max(ASPECT_RATIO_X / window.width());
        transform.scale = Vec3::new(scale, scale, 1.0);
    }
}

#[derive(Debug, Default)]
pub struct CameraPlugin {}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, auto_scale_sys);
    }
}
