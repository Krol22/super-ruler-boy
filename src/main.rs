use bevy::{prelude::{App, default, Commands, ResMut, Assets, Res, AssetServer, Vec2, SpatialBundle, Vec3, Transform, BuildChildren, Startup, Query, Children, With, Update, IntoSystemConfigs, KeyCode, Input, Rect, Component, Without, Bundle, Entity}, DefaultPlugins, window::{WindowPlugin, Window, WindowResolution}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite, SpriteBundle, Sprite}, utils::HashMap, transform::TransformBundle, time::{Time, Timer, TimerMode}};
use bevy::prelude::PluginGroup;

use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection, LdtkIntCell, IntGridCell, prelude::{LdtkIntCellAppExt, LdtkEntityAppExt}, LdtkEntity, EntityInstance};
use bevy_rapier2d::{prelude::{RigidBody, Collider, KinematicCharacterController, Sensor, QueryFilterFlags, RapierContext, QueryFilter}};
use kt_common::{CommonPlugin, components::{limb::{Limb, LimbType}, player::Player, jump::Jump, gravity::GravityDir, velocity::Velocity, acceleration::Acceleration, checkpoint::Checkpoint}};
use kt_core::{CorePlugin, animation::{Animation, Animator, animator_sys}};
use kt_movement::MovementPlugin;
use kt_util::constants::{WINDOW_TITLE, INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT, PLAYER_HIT_RESPAWN_TIME};

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
        .add_plugins(LdtkPlugin)
        .add_plugins(CommonPlugin {})
        .add_plugins(CorePlugin {})
        .add_plugins(MovementPlugin {})
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_entity::<SpikesBundle>("Spikes")
        .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
        .add_systems(Update, setup_walls)
        .add_systems(Update, (
    handle_animation,
    animator_sys,
    handle_extension_stretch,
    handle_stretching,
    flip_depend_on_velocity,
    handle_player_hurt_collision,
    handle_activate_checkpoint,
    checkpoint_sprites_handle,
    respawn_player,
).chain()).run();
}

fn handle_activate_checkpoint(
    q_player: Query<(&Transform, &Velocity), With<Player>>,
    mut q_checkpoints: Query<&mut Checkpoint>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (transform, velocity) in q_player.iter() {
        let shape = Collider::cuboid(6.0, 9.0);
        let shape_pos = transform.translation.truncate();
        let shape_vel = Vec2::new(velocity.current.x * time.delta_seconds(), velocity.current.y * time.delta_seconds());
        let shape_rot = 0.0;
        let max_toi = 1.0;
        let filter = QueryFilter {
            flags: QueryFilterFlags::EXCLUDE_SOLIDS,
            ..default()
        };

        if let Some((entity, _hit)) = rapier_context.cast_shape(
            shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
        ) {
            let checkpoint = q_checkpoints.get(entity);

            match checkpoint {
                Ok(checkpoint) => {
                    if checkpoint.is_active {
                        continue;
                    }
                },
                Err(..) => continue,
            };

            for mut checkpoint in q_checkpoints.iter_mut() {
                checkpoint.is_active = false;
            }

            let checkpoint_to_activate = q_checkpoints.get_mut(entity);
            match checkpoint_to_activate {
                Ok(mut checkpoint_to_activate) => checkpoint_to_activate.is_active = true,
                Err(..) => continue,
            }
        }
    }
}

fn checkpoint_sprites_handle(
    mut q_checkpoints: Query<(&mut TextureAtlasSprite, &Checkpoint)>,
) {
    for (mut sprite, checkpoint) in q_checkpoints.iter_mut() {
        if checkpoint.is_active {
            sprite.index = 2;
        } else {
            sprite.index = 1;
        }
    }
}

fn respawn_player(
    mut q_player: Query<(&mut Transform, &mut Player)>,
    q_checkpoint: Query<(&Transform, &Checkpoint), Without<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in q_player.iter_mut() {
        player.respawn_timer.tick(time.delta());

        if player.respawn_timer.just_finished() {
            let mut checkpoint_position: Vec2 = Vec2::ZERO;

            for (checkpoint_transform, checkpoint) in q_checkpoint.iter() {
                if checkpoint.is_active {
                    checkpoint_position = checkpoint_transform.translation.truncate();
                    break;
                }
            }

            transform.translation.x = checkpoint_position.x;
            transform.translation.y = checkpoint_position.y;
        }
    }
}

fn handle_player_hurt_collision(
    mut q_player: Query<(&mut Transform, &Velocity, &mut Player)>,
    q_hit: Query<&HitComponent>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (transform, velocity, mut player) in q_player.iter_mut() {

        let shape = Collider::cuboid(6.0, 9.0);
        let shape_pos = transform.translation.truncate();
        let shape_vel = Vec2::new(velocity.current.x * time.delta_seconds(), velocity.current.y * time.delta_seconds());
        let shape_rot = 0.0;
        let max_toi = 1.0;
        let filter = QueryFilter {
            flags: QueryFilterFlags::EXCLUDE_SOLIDS,
            ..default()
        };

        if let Some((entity, hit)) = rapier_context.cast_shape(
            shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
        ) {
            let hit_component = q_hit.get(entity);
            dbg!(hit_component);
            if hit_component.is_ok() {
                player.respawn_timer = Timer::from_seconds(PLAYER_HIT_RESPAWN_TIME, TimerMode::Once);
            }
            continue
        }
    }
}

fn setup_walls(
    mut commands: Commands,
    q_walls: Query<(&Transform, Entity), With<WallDefinition>>,
) {
    let mut points: Vec<Point> = vec![];

    for (transform, entity) in q_walls.iter() {
        commands
            .entity(entity)
            .remove::<RigidBody>()
            .remove::<Collider>()
            .remove::<WallDefinition>();

        points.push(
            Point {
                x: (transform.translation.x / 16.0) as i32,
                y: (transform.translation.y / 16.0) as i32,
            }
        );
    }

    if points.is_empty() {
        return;
    }

    dbg!(points.len());
    let points_with_neighbors = find_points_with_neighbors(&points);
    let neighbor_set: std::collections::HashSet<_> = points_with_neighbors.into_iter().collect();
    points.retain(|p| !neighbor_set.contains(p));

    let rects = find_rectangles(&points);

    for rect in rects.iter() {
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                rect.top_left.x as f32 * 16.0 + rect.width as f32 * 16.0 / 2.0,
                rect.top_left.y as f32 * 16.0 + 8.0, 0.0
            )),
            RigidBody::Fixed,
            Collider::cuboid(8.0 * rect.width as f32, 8.0),
        ));
    }
}

fn find_points_with_neighbors(points: &[Point]) -> Vec<Point> {
    let mut result = Vec::new();

    for point in points {
        let has_left_neighbor = points.iter().any(|p| p.x == point.x - 1 && p.y == point.y);
        let has_right_neighbor = points.iter().any(|p| p.x == point.x + 1 && p.y == point.y);
        let has_top_neighbor = points.iter().any(|p| p.y == point.y + 1 && p.x == point.x);
        let has_bottom_neighbor = points.iter().any(|p| p.y == point.y - 1 && p.x == point.x);

        if has_left_neighbor && has_right_neighbor && has_top_neighbor && has_bottom_neighbor {
            result.push(*point);
        }
    }

    result
}

fn find_rectangles(points: &[Point]) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    let mut sorted_points = points.to_vec();
    sorted_points.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

    for i in 0..sorted_points.len() {
        let point = sorted_points[i];
        // Skip if this point is already part of a rectangle
        if rectangles.iter().any(|r| is_point_in_rectangle(r, &point)) {
            continue;
        }

        // Find the maximum width
        let mut width = 1;
        while i + width < sorted_points.len() && sorted_points[i + width].y == point.y && sorted_points[i + width].x == point.x + width as i32 {
            width += 1;
        }

        // Add the rectangle
        rectangles.push(Rectangle {
            top_left: point,
            width,
            height: 1,
        });
    }

    rectangles
}

fn is_point_in_rectangle(rect: &Rectangle, point: &Point) -> bool {
    point.x >= rect.top_left.x
        && point.x < rect.top_left.x + rect.width as i32
        && point.y >= rect.top_left.y
        && point.y < rect.top_left.y + rect.height as i32
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct CheckpointBundle {
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub checkpoint: Checkpoint,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Clone, Component, Debug, Default)]
pub struct HitComponent {}

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpikesBundle {
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub hit_component: HitComponent,
}

#[derive(Clone, Component, Debug, Default)]
pub struct WallDefinition {}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    #[from_int_grid_cell]
    pub collider_bundle: ColliderBundle,
    pub wall: WallDefinition,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        if int_grid_cell.value == 1 {
            ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Fixed,
            }
        } else {
            ColliderBundle::default()
        }
    }
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(_entity_instance: &EntityInstance) -> ColliderBundle {
        ColliderBundle {
            collider: Collider::cuboid(2., 2.),
            rigid_body: RigidBody::Fixed,
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
}

impl From<&EntityInstance> for SensorBundle {
    fn from(entity_instance: &EntityInstance) -> SensorBundle {
        dbg!(&entity_instance.identifier);

        match entity_instance.identifier.as_ref() {
            "Spikes" => SensorBundle {
                collider: Collider::cuboid(8.0, 8.0), // #TODO pull out from editor
                ..default()
            },
            "Checkpoint" => SensorBundle {
                collider: Collider::cuboid(8.0, 8.0),
                ..default()
            },
            _ => SensorBundle::default()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rectangle {
    top_left: Point,
    width: usize,
    height: usize,
}

// fn merge_tiles_into_shapes(tiles: Vec<Point>, cell_size: i32) -> Vec<Rectangle> {
        // let mut shapes = Vec::new();
    // let mut visited = std::collections::HashSet::new();

    // // Sort tiles by y (ascending), then by x (ascending) to start from the top left tile
    // let mut sorted_tiles = tiles.clone();
    // sorted_tiles.sort_by_key(|k| (k.y, k.x));

    // for &tile in &sorted_tiles {
        // if visited.contains(&tile) {
            // continue; // Skip already visited tiles
        // }

        // // Start a new shape from this tile
        // let mut shape = Rectangle {
            // top_left: tile,
            // width: cell_size,
            // height: cell_size,
        // };

        // // Try to expand the shape to the right as far as possible
        // while sorted_tiles.contains(&Point {
            // x: shape.top_left.x + shape.width,
            // y: tile.y,
        // }) {
            // shape.width += cell_size;
        // }

        // // Try to expand the shape upwards (increasing y) as far as possible
        // while sorted_tiles.iter().any(|&p| {
    // let in_x_range = (shape.top_left.x..shape.top_left.x + shape.width)
        // .step_by(cell_size as usize)
        // .any(|x| x == p.x);
    // in_x_range && p.y == shape.top_left.y + shape.height
// }) {
            // shape.height += cell_size;
        // }

        // // Add tiles to visited within the bounds of the shape
        // for y in (tile.y..tile.y + shape.height).step_by(cell_size as usize) {
            // for x in (tile.x..tile.x + shape.width).step_by(cell_size as usize) {
                // visited.insert(Point { x, y });
            // }
        // }

        // shapes.push(shape);
    // }

    // shapes
// }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("world.ldtk"),
        ..default()
    });
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

    let mut respawn_timer = Timer::from_seconds(PLAYER_HIT_RESPAWN_TIME, TimerMode::Once);
    respawn_timer.pause();

    let player = commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 100.0, 0.0)),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(6.0, 9.0),
        GravityDir {
            dir: 0,
        },
        Velocity {
            damping: 0.05,
            ..default()
        },
        Acceleration::default(),
        Jump::default(),
        KinematicCharacterController {
            filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
            ..default()
        },
        Player {
            respawn_timer,
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

    // commands.spawn((
        // RigidBody::Fixed,
        // Collider::cuboid(2000.0, 10.0),
        // TransformBundle::from_transform(Transform::from_xyz(-1000.0, -40.0, 0.0)),
    // ));

    // commands.spawn((
        // RigidBody::Fixed,
        // Collider::cuboid(40.0, 10.0),
        // TransformBundle::from_transform(Transform::from_xyz(-60.0, 20.0, 0.0)),
    // ));

    // commands.spawn((
        // RigidBody::Fixed,
        // Collider::cuboid(40.0, 10.0),
        // TransformBundle::from_transform(Transform::from_xyz(60.0, 20.0, 0.0)),
    // ));
}
