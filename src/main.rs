use bevy::{prelude::{App, default, Commands, ResMut, Assets, Res, AssetServer, Vec2, SpatialBundle, Vec3, Transform, BuildChildren, Startup, Query, Children, With, Update, IntoSystemConfigs, KeyCode, Input, Rect, Component, Without, Entity}, DefaultPlugins, window::{WindowPlugin, Window, WindowResolution}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite, SpriteBundle, Sprite}, utils::HashMap, transform::TransformBundle, time::Time};
use bevy::prelude::PluginGroup;

use bevy_rapier2d::{prelude::{RigidBody, Collider, KinematicCharacterController, Velocity, ExternalImpulse, KinematicCharacterControllerOutput, FixedJointBuilder, ImpulseJoint, CollisionGroups, Group, RapierContext, QueryFilter, CharacterLength, QueryFilterFlags}, rapier::prelude::InteractionGroups};
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
        .add_systems(Update, (handle_animation, animator_sys, handle_extension_stretch, handle_stretching).chain())
        .add_systems(Update, (cast_rays, controls, handle_stretching_controls, velocity_y, /* move_body_shape */ sync_x_ceiling_sensor).chain())
        // .add_systems(Update, (sync_y_diff).chain())
        .run();
}


#[derive(Debug, Component)]
struct VelocityY(f32);

#[derive(Debug, Component)]
struct CeilingSensor {
    has_x_collision: bool,
}

#[derive(Debug, Component)]
struct PlayerLimbs {}

#[derive(Debug, Default, Component)]
struct GravityDir {
    dir: isize,
}

fn cast_rays (
    mut q_player: Query<(&Transform, &mut Player, Entity)>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, mut player, Entity) in q_player.iter_mut() {
        let ray_pos = transform.translation.truncate();
        let ray_dir = Vec2::new(2.0, 0.0);
        let max_toi = 4.0;
        let solid = true;
        let filter = QueryFilter {
            exclude_collider: Some(Entity),
            flags: QueryFilterFlags::ONLY_FIXED, 
            ..default()
        };

        player.has_x_collision = 0;
        if let Some((entity, toi)) = rapier_context.cast_ray(
            ray_pos, ray_dir, max_toi, solid, filter
        ) {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.
            let hit_point = ray_pos + ray_dir * toi;
            println!("Entity {:?} hit at point {}", entity, hit_point);
            player.has_x_collision = 1;
            continue;
        }

        let ray_pos = transform.translation.truncate();
        let ray_dir = Vec2::new(-2.0, 0.0);
        let max_toi = 4.0;
        let solid = true;
        let filter = QueryFilter {
            exclude_collider: Some(Entity),
            flags: QueryFilterFlags::ONLY_FIXED, 
            ..default()
        };

        if let Some((entity, toi)) = rapier_context.cast_ray(
            ray_pos, ray_dir, max_toi, solid, filter
        ) {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.
            let hit_point = ray_pos + ray_dir * toi;
            println!("Entity {:?} hit at point {}", entity, hit_point);
            player.has_x_collision = -1;
            continue;
        }
    }
}

fn sync_x_ceiling_sensor (
    mut q_player: Query<(&KinematicCharacterControllerOutput, &mut Transform, &mut Player), (With<Player>, Without<CeilingSensor>)>,
    mut q_ceiling_sensor: Query<(&KinematicCharacterControllerOutput, &mut Transform, &mut CeilingSensor), (With<CeilingSensor>, Without<Player>)>,
) {
    if q_player.iter().count() == 0 || q_ceiling_sensor.iter().count() == 0 {
        return;
    }

    let (kpo, mut player_transform, mut player) = q_player.get_single_mut().unwrap();
    let (kclo, mut ceiling_sensor_transform, mut ceiling_sensor) = q_ceiling_sensor.get_single_mut().unwrap();

    let mut has_x_collision = false;

    for collision in &kclo.collisions {
        if collision.translation_remaining.x > 0.001 {
            has_x_collision = true;
        }
    }
    
    if has_x_collision {
        player_transform.translation.x = ceiling_sensor_transform.translation.x;
    }

    let mut has_x_collision = false;

    for collision in &kpo.collisions {
        if collision.translation_remaining.x > 0.001 {
            has_x_collision = true;
        }
    }

    if has_x_collision {
        ceiling_sensor_transform.translation.x = player_transform.translation.x;
    }
}

fn handle_stretching_controls(
    mut q_ceiling_sensor: Query<(&Transform, &mut GravityDir), (With<CeilingSensor>, Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<CeilingSensor>)>,
    q_kpo: Query<&KinematicCharacterControllerOutput, (With<Player>, Without<CeilingSensor>)>,
    q_kcso: Query<&KinematicCharacterControllerOutput, (With<CeilingSensor>, Without<Player>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player_transform = q_player.get_single().unwrap();

    if q_kpo.iter().count() == 0 {
        return;
    }

    if q_kcso.iter().count() != 0 {
        let kcso = q_kcso.get_single().unwrap();
        // dbg!(kcso);
    }

    let kpo = q_kpo.get_single().unwrap();

    for (transform, mut ceiling_sensor) in q_ceiling_sensor.iter_mut() {
        if kpo.grounded {
            let mut dir = 1;
            if player_transform.translation.distance(transform.translation) > 30.0 && keyboard_input.pressed(KeyCode::Space) {
                dir = 0;
                ceiling_sensor.dir = dir;
                continue;
            } 

            if keyboard_input.pressed(KeyCode::Space) {
                dir = -1;
            }

            ceiling_sensor.dir = dir;
            continue;
        }

        let is_falling = kpo.effective_translation.y < 0.0;

        if is_falling {
            if keyboard_input.pressed(KeyCode::Space) && player_transform.translation.distance(transform.translation) < 30.0 {
                ceiling_sensor.dir = -1;
                continue;
            }

            if player_transform.translation.distance(transform.translation) > 30.0 {
                ceiling_sensor.dir = 2;
                continue;
            }

            if player_transform.translation.distance(transform.translation) < 0.5 {
                ceiling_sensor.dir = 1;
                continue;
            }
        } else {
            println!("Jumping not implemented yet!");
        }
    }
}

fn handle_stretching(
    q_player_limbs_container: Query<&Children, (With<PlayerLimbs>, Without<CeilingSensor>)>,    
    q_ceiling_sensor: Query<&Transform, (With<CeilingSensor>, Without<Limb>)>,
    mut q_player_limbs: Query<(&mut Transform, &Limb)>,
) {
    let ceiling_sensor_transform = q_ceiling_sensor.get_single().unwrap();

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

            // transform.translation.y = ceiling_sensor_transform.translation.y + 5.0;
        }
    }
}

fn handle_extension_stretch(
    q_player: Query<(&Children, &Player)>,    
    mut q_player_limbs: Query<(&mut Sprite, &Limb, &mut Transform)>,
    mut q_player_limbs_2: Query<(&Animator, &Limb)>,
) {
    let mut extension_offset = HashMap::new();
    extension_offset.insert("Idle".to_string(), vec![0, -1, -1, 0]);
    extension_offset.insert("Move".to_string(), vec![0, 1, 3, 1, 0, 1, 3, 1, 0]);

    let mut extension_frame = HashMap::new();
    extension_frame.insert("Idle".to_string(), vec![0, 0, 0, 0]);
    extension_frame.insert("Move".to_string(), vec![0, 0, 0, 0, 1, 1, 1, 1]);

    for (children, player) in q_player.iter() {
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

            sprite.rect = Some(Rect::new(frame as f32 * 12.0, 0.0, 12.0 + 12.0 * frame as f32, player.stretch + 4.0));
            transform.translation.y = (player.stretch / 2.0) - 7.0 + offset as f32;
        }
    }
}

fn controls (
    mut q_player: Query<(&mut KinematicCharacterController, &Player), Without<CeilingSensor>>,
    mut q_ceiling_sensor: Query<(&mut KinematicCharacterController, &CeilingSensor), Without<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut impulse, player) = q_player.get_single_mut().unwrap();
    let (mut kcc, celing_sensor) = q_ceiling_sensor.get_single_mut().unwrap();

    let mut translation_x = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        translation_x = -1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        translation_x = 1.0;
    }

    impulse.translation = Some(Vec2::new(translation_x, 0.0));
    if player.has_x_collision < 0 && translation_x < 0.0 {
        return;
    } else if player.has_x_collision > 0 && translation_x > 0.0 {
        return;
    }

    kcc.translation = Some(Vec2::new(translation_x, 0.0));
}

fn velocity_y(
    mut player_query: Query<(&mut KinematicCharacterController, &GravityDir)>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        return;
    }

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

#[repr(u32)]
pub enum NamedCollisionGroups {
	Everything = std::u32::MAX,
	Terrain = 0b0001,
	Projectile = 0b0010,
	Npc = 0b0100,
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
        Collider::cuboid(6.0, 11.0),
        GravityDir {
            dir: 1,
        },
        KinematicCharacterController {
            filter_groups: Some(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1)),
            ..default()
        },
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
        Player {
            ..default()
        },
    )).id();

    let player_limbs = commands.spawn((
        PlayerLimbs {},
        SpatialBundle::from_transform(Transform::from_xyz(320.0, 8.0, 0.0)),
    )).id();

    commands.spawn((
        CeilingSensor { has_x_collision: false },
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(6.0, 11.0),
        GravityDir {
            dir: 1,
        },
        KinematicCharacterController {
            filter_groups: Some(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1)),
            ..default()
        },
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 100.0, 0.0)),
    ));

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
        TransformBundle::from_transform(Transform::from_xyz(80.0, -0.0, 0.0)),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(60.0, 40.0, 0.0)),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(-80.0, -40.0, 0.0)),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(-120.0, -25.0, 0.0)),
    ));
}
