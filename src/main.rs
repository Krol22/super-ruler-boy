use bevy::{prelude::{App, default, Commands, ResMut, Assets, Res, AssetServer, Vec2, SpatialBundle, Vec3, Transform, BuildChildren, Startup, Query, Children, With, Update, IntoSystemConfigs, KeyCode, Input, Rect, Component, Without}, DefaultPlugins, window::{WindowPlugin, Window, WindowResolution}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite, SpriteBundle, Sprite}, utils::HashMap, transform::TransformBundle};
use bevy::prelude::PluginGroup;

use bevy_rapier2d::prelude::{RigidBody, Collider, KinematicCharacterController};
use kt_common::{CommonPlugin, components::{limb::{Limb, LimbType}, player::Player, jump::Jump, gravity::GravityDir, velocity::Velocity, acceleration::Acceleration}};
use kt_core::{CorePlugin, animation::{Animation, Animator, animator_sys}};
use kt_movement::MovementPlugin;
use kt_util::constants::{WINDOW_TITLE, INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
              title: WINDOW_TITLE.to_string(),
              resizable: true,
              resolution: WindowResolution::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT),
              ..default()
            }),
            ..default()
        }))
        .add_plugins(CommonPlugin {})
        .add_plugins(CorePlugin {})
        .add_plugins(MovementPlugin {})
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
    handle_animation,
    animator_sys,
    handle_extension_stretch,
    handle_stretching,
    flip_depend_on_velocity,
).chain()).run();
}

#[derive(Debug, Component)]
struct PlayerLimbs {}

fn handle_stretching(
    q_player_limbs_container: Query<&Children, With<PlayerLimbs>>,    
    mut q_player_limbs: Query<(&mut Transform, &Limb)>,
    q_player: Query<&Player, Without<PlayerLimbs>>,
) {
    let player = q_player.get_single().unwrap();

    for children in q_player_limbs_container.iter() {
        for &child in children.iter() {
            let child = q_player_limbs.get_mut(child);

            let (mut transform, limb) = match child {
                Ok(child) => child,
                Err(..) => continue,
            };

            match limb.limb_type {
                LimbType::Body => (),
                _ => continue,
            }

            transform.translation.y = player.stretch;
        }
    }
}

fn handle_extension_stretch(
    q_player_children: Query<(&Children, &PlayerLimbs)>,    
    q_player: Query<&Player>,    
    mut q_player_limbs: Query<(&mut Sprite, &Limb, &mut Transform)>,
    mut q_player_limbs_2: Query<(&Animator, &Limb)>,
) {
    let mut extension_offset = HashMap::new();
    extension_offset.insert("Idle".to_string(), vec![0, -1, -1, 0]);
    extension_offset.insert("Move".to_string(), vec![0, 1, 3, 1, 0, 1, 3, 1, 0]);

    let mut extension_frame = HashMap::new();
    extension_frame.insert("Idle".to_string(), vec![0, 0, 0, 0]);
    extension_frame.insert("Move".to_string(), vec![0, 0, 0, 0, 1, 1, 1, 1]);

    for (children, player) in q_player_children.iter() {
        let mut offset = 0;
        let mut frame = 0;

        for &child in children.iter() {
            let child = q_player_limbs_2.get_mut(child);

            let (animator, limb) = match child {
                Ok(child) => child,
                Err(..) => continue,
            };

            match limb.limb_type {
                LimbType::Body => (),
                _ => continue,
            }

            // dbg!(animator.current_frame);
            let offsets = extension_offset.get(&animator.current_animation);
            let offsets = match offsets {
                Some(offsets) => offsets,
                None => continue,
            };

            let frames = extension_frame.get(&animator.current_animation);
            let frames = match frames {
                Some(frames) => frames,
                None => continue,
            };

            offset = offsets[animator.current_frame];
            frame = frames[animator.current_frame];
        }

        for &child in children.iter() {
            let child = q_player_limbs.get_mut(child);

            let (mut sprite, limb, mut transform) = match child {
                Ok(child) => child,
                Err(..) => continue,
            };

            match limb.limb_type {
                LimbType::Extension => (),
                _ => continue,
            }

            let player = q_player.get_single().unwrap();

            sprite.rect = Some(Rect::new(frame as f32 * 12.0, 0.0, 12.0 + 12.0 * frame as f32, player.stretch + 4.0));
            transform.translation.y = (player.stretch / 2.0) - 7.0 + offset as f32;
        }
    }
}

fn flip_depend_on_velocity(
    mut q_entities: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in q_entities.iter_mut() {
        if velocity.current.x < -0.1 {
            // transform.scale.x = -1.0;
        }

        if velocity.current.x > 0.1 {
            // transform.scale.x = 1.0;
        }
    }
}

fn handle_animation(
    q_player: Query<(&Velocity, &Children), With<Player>>,    
    mut q_player_limbs: Query<(&Children, &mut Transform), With<PlayerLimbs>>,
    mut q_limbs: Query<&mut Animator, With<Limb>>,
) {
    for (velocity, children) in q_player.iter() {
        let mut animation_name = "Idle".to_string();

        if velocity.current.x < -0.1  || velocity.current.x > 0.1 {
            animation_name = "Move".to_string();
        }

        for &child in children.iter() {
            let child = q_player_limbs.get_mut(child);

            let (player_limbs, mut transform) = match child {
                Ok(child) => child,
                Err(..) => continue,
            };

            // #TODO - This could be tweened
            if velocity.current.x < -0.1 {
                transform.scale.x = -1.0;
            } else if velocity.current.x > 0.1 {
                transform.scale.x = 1.0; 
            }

            for &limb in player_limbs.iter() {
                let player_limb = q_limbs.get_mut(limb);

                let mut animator = match player_limb {
                    Ok(player_limb) => player_limb,
                    Err(..) => continue,
                };

                animator.current_animation = animation_name.clone();
            }

        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/ruler_spirite.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(19.0, 22.0),
        13,
        3,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 100.0, 0.0)),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(6.0, 9.0),
        GravityDir {
            dir: 1,
        },
        Velocity::default(),
        Acceleration::default(),
        Jump::default(),
        KinematicCharacterController::default(),
        Player {
            ..default()
        },
    )).id();

    let player_limbs = commands.spawn((
        PlayerLimbs {},
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 2.0, 0.0)),
    )).id();

    // Spawn legs
    let mut legs_animations = HashMap::new();

    let legs_idle_animation = Animation {
        frames: vec![0, 1, 2, 3],
        looping: true,
        fps: 5,
    };

    let legs_move_animation = Animation {
        frames: (4..12).collect(),
        looping: true,
        fps: 7,
    };

    let legs_extending_animation = Animation {
        frames: vec![0],
        looping: true,
        fps: 1,
    };

    legs_animations.insert("Idle".to_string(), legs_idle_animation);
    legs_animations.insert("Move".to_string(), legs_move_animation);
    legs_animations.insert("Extending".to_string(), legs_extending_animation);

    let legs = commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Animator {
            animations: legs_animations,
            counter: 0.0,
            current_animation: "Idle".to_string(),
            prev_animation: "Idle".to_string(),
            current_frame: 0,
        },
        Limb::new(LimbType::Legs)
    )).id();

    // Spawn body
    let mut body_animations = HashMap::new();

    let body_idle_animation = Animation {
        frames: vec![13, 14, 15, 16],
        looping: true,
        fps: 5,
    };

    let body_move_animation = Animation {
        frames: (17..25).collect(),
        looping: true,
        fps: 7,
    };

    let body_extending_animation = Animation {
        frames: vec![13],
        looping: true,
        fps: 1,
    };

    body_animations.insert("Idle".to_string(), body_idle_animation);
    body_animations.insert("Move".to_string(), body_move_animation);
    body_animations.insert("Extending".to_string(), body_extending_animation);

    let body = commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(13),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Animator {
            animations: body_animations,
            counter: 0.0,
            current_animation: "Idle".to_string(),
            prev_animation: "Idle".to_string(),
            current_frame: 0,
        },
        Limb::new(LimbType::Body)
    )).id();

    // Spawn hands
    let mut hands_animations = HashMap::new();

    let hands_idle_animation = Animation {
        frames: vec![26, 27, 28, 29],
        looping: true,
        fps: 5,
    };

    let hands_move_animation = Animation {
        frames: (30..38).collect(),
        looping: true,
        fps: 7,
    };

    let hands_extending_animation = Animation {
        frames: vec![26],
        looping: true,
        fps: 1,
    };

    hands_animations.insert("Idle".to_string(), hands_idle_animation);
    hands_animations.insert("Move".to_string(), hands_move_animation);
    hands_animations.insert("Extending".to_string(), hands_extending_animation);

    let hands = commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(13),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Animator {
            animations: hands_animations,
            counter: 0.0,
            current_animation: "Idle".to_string(),
            prev_animation: "Idle".to_string(),
            current_frame: 0,
        },
        Limb::new(LimbType::Hands)
    )).id();

    // Spawn extension
    let extension = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ruler_extension_part.png"),
            transform: Transform::from_xyz(0.5, 0., 0.),
            sprite: Sprite {
                rect: Some(Rect::new(0.0, 0.0, 12.0, 0.0)),
                ..default()
            },
            ..default()
        },
        Limb::new(LimbType::Extension),
    )).id();

    // Attach limbs to player xD
    commands.entity(player_limbs)
        .add_child(extension)
        .add_child(legs)
        .add_child(body)
        .add_child(hands);

    commands.entity(player)
        .add_child(player_limbs);

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(2000.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(-1000.0, -40.0, 0.0)),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(-60.0, 20.0, 0.0)),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(60.0, 20.0, 0.0)),
    ));
}
