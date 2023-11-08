use bevy::{prelude::{App, default, Commands, ResMut, Assets, Res, AssetServer, Vec2, SpatialBundle, Vec3, Transform, BuildChildren, Startup, Query, Children, With, Update, IntoSystemConfigs, KeyCode, Input, Rect, Component, Without, Entity}, DefaultPlugins, window::{WindowPlugin, Window, WindowResolution}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite, SpriteBundle, Sprite}, utils::HashMap, transform::TransformBundle, time::Time};
use bevy::prelude::PluginGroup;

use bevy_rapier2d::{prelude::{RigidBody, Collider, KinematicCharacterController, ExternalImpulse, KinematicCharacterControllerOutput, FixedJointBuilder, ImpulseJoint, CollisionGroups, Group, RapierContext, QueryFilter, CharacterLength, QueryFilterFlags}, rapier::prelude::InteractionGroups};
use kt_common::{CommonPlugin, components::{limb::{Limb, LimbType}, player::Player}};
use kt_core::{CorePlugin, animation::{Animation, Animator, animator_sys}};
use kt_movement::MovementPlugin;
use kt_util::constants::{WINDOW_TITLE, INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT, self};

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
        .add_systems(Update, (controls_when_grabbed_ceiling, controls, controls_2, velocity_y, handle_animation, animator_sys, handle_extension_stretch, handle_stretching, grab_ceiling, ungrab_ceiling).chain())
        .run();
}


#[derive(Debug, Component, Default)]
pub struct Velocity {
    pub current: Vec2,
}

#[derive(Debug, Component)]
struct PlayerLimbs {}

#[derive(Debug, Default, Component)]
struct GravityDir {
    dir: isize,
}

fn ungrab_ceiling(
    mut q_player: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Z) {
        for mut player in q_player.iter_mut() {
            player.grabbed_ceiling = false;
        }
    }
}

fn grab_ceiling(
    mut q_player: Query<(&mut Player, &mut GravityDir)>,
) {
    for (mut player, mut gravity_dir) in q_player.iter_mut() {
        if player.grabbed_ceiling {
            gravity_dir.dir = -1;
            player.stretch -= 1.0;
        } else {
            gravity_dir.dir = 1;
        }

        if player.stretch <= 0.0 {
            player.stretch = 0.0;
        }
    }
}


fn controls_2(
    mut q_player: Query<(&Transform, &mut Player)>,
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (transform, mut player) in q_player.iter_mut() {
        if player.grabbed_ceiling {
            continue;
        }

        if keyboard_input.pressed(KeyCode::Space) {

            if player.stretch >= 40.0 {
                player.stretch = 40.0;
                continue;
            }

            let ray_pos = transform.translation.truncate();
            let ray_dir = Vec2::new(-1.1, player.stretch / 3.0 - 0.3);
            let max_toi = 4.0;
            let solid = true;
            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED, 
                ..default()
            };

            player.grabbed_ceiling = false;
            if let Some(_entity) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            ) {
                player.grabbed_ceiling = true;
                continue;
            }

            let ray_pos = transform.translation.truncate();
            let ray_dir = Vec2::new(1.1, player.stretch / 3.0 - 0.3);
            let max_toi = 4.0;
            let solid = true;
            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED, 
                ..default()
            };

            if let Some(_entity) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            ) {
                player.grabbed_ceiling = true;
                continue;
            }

            player.stretch += 1.0;
            continue;
        }

        player.stretch -= 1.0;

        if player.stretch < 0.0 {
            player.stretch = 0.0;
        }
    }

}

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

fn controls_when_grabbed_ceiling(
    mut q_player: Query<(&mut KinematicCharacterController, &Player, &Transform)>,
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player = q_player.get_single_mut();
    let (mut kcc, player, transform) = match player {
        Ok(player) => player,
        Err(..) => return,
    };

    if !player.grabbed_ceiling {
        return;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        let ray_pos = transform.translation.truncate();
        let ray_dir = Vec2::new(-0.1, 6.0);
        let max_toi = 4.0;
        let solid = true;
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_FIXED, 
            ..default()
        };

        if let Some(_entity) = rapier_context.cast_ray(
            ray_pos, ray_dir, max_toi, solid, filter
        ) {
            kcc.translation = Some(Vec2::new(-0.2, 0.0));
        }

        return;
    } 

    if keyboard_input.pressed(KeyCode::Right) {
        let ray_pos = transform.translation.truncate();
        let ray_dir = Vec2::new(0.1, 6.0);
        let max_toi = 4.0;
        let solid = true;
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_FIXED, 
            ..default()
        };

        if let Some(_entity) = rapier_context.cast_ray(
            ray_pos, ray_dir, max_toi, solid, filter
        ) {
            kcc.translation = Some(Vec2::new(0.2, 0.0));
        }
    }
}

fn controls (
    mut q_player: Query<(&mut KinematicCharacterController, &Player, &Transform)>,
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut kcc, player, transform) in q_player.iter_mut() {
        if player.grabbed_ceiling {
            continue;
        }

        let mut translation_x = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            let ray_pos = transform.translation.truncate() + Vec2::new(0.0, player.stretch + 1.3);
            let ray_dir = Vec2::new(-1.3, 0.0);
            let max_toi = 4.0;
            let solid = true;
            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED, 
                ..default()
            };

            if let Some(_entity) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            ) {
                continue;
            }

            translation_x = -1.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            let ray_pos = transform.translation.truncate() + Vec2::new(0.0, player.stretch + 1.3);
            let ray_dir = Vec2::new(1.3, 0.0);
            let max_toi = 4.0;
            let solid = true;
            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED, 
                ..default()
            };

            if let Some(_entity) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            ) {
                continue;
            }

            translation_x = 1.0;
        }

        kcc.translation = Some(Vec2::new(translation_x, 0.0));
    }
}

fn velocity_y(
    mut player_query: Query<(&mut KinematicCharacterController, &GravityDir)>,
    time: Res<Time>,
) {
    for (mut player, gravity_dir) in player_query.iter_mut() {
        let mut transform = Vec2::ZERO;
        transform.y += time.delta_seconds() * -90.0 * gravity_dir.dir as f32;

        match player.translation {
            Some(t) => player.translation = Some(transform + t),
            None => player.translation = Some(transform),
        }
    }
}

fn handle_animation(
    q_player: Query<(&Player, &Children)>,    
    mut q_player_limbs: Query<(&mut Animator, &Limb)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    return;
    for (player, children) in q_player.iter() {
        // if player.stretch != 0.0 {
            // for &child in children.iter() {
                // let child = q_player_limbs.get_mut(child);

                // let (mut animator, limb) = match child {
                    // Ok(child) => child,
                    // Err(..) => continue,
                // };

                // animator.current_animation = "Extending".to_string();
            // }
        // } 

        if keyboard_input.pressed(KeyCode::Left) {
            let animation_name = "Move".to_string();

            for &child in children.iter() {
                let child = q_player_limbs.get_mut(child);

                let (mut animator, limb) = match child {
                    Ok(child) => child,
                    Err(..) => continue,
                };

                animator.current_animation = animation_name.clone();
            }
        }

        if keyboard_input.pressed(KeyCode::Right) {
            let animation_name = "Idle".to_string();

            for &child in children.iter() {
                let child = q_player_limbs.get_mut(child);

                let (mut animator, limb) = match child {
                    Ok(child) => child,
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
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -25.0, 0.0)),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(-60.0, 40.0, 0.0)),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(60.0, 40.0, 0.0)),
    ));
}
