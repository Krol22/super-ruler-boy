use std::time::Duration;

use bevy::{prelude::{Query, Transform, Entity, Commands, Res, AssetServer, Added, Vec3, SpatialBundle, With, Without, Color, default, Vec2, BuildChildren, Image, ResMut, EventWriter, Assets}, sprite::{SpriteBundle, Sprite, TextureAtlas, TextureAtlasSprite, SpriteSheetBundle}, render::render_resource::Texture, time::{Timer, TimerMode}};
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor, GravityScale};
use bevy_tweening::{Tween, EaseFunction, lens::{TransformPositionLens, SpriteColorLens}, RepeatCount};
use kt_common::{components::{platform::Platform, despawnable::Despawnable, ldtk::{ElevatorInstance, SpawnPoint, WallDefinition, PointTo, Elevator, Level, PlatformInstance, SharpenerInstance, PinInstance, ExitBundle, ExitInstance, RequiredKeys, Exit}, player::Player, pin::Pin, sharpener::Sharpener, interaction::Interaction}, events::PinUiUpdated};
use kt_util::constants::{Z_INDEX_PENCIL_BOX, PLAYER_HIT_RESPAWN_TIME};

use crate::save_game::GameState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub top_left: Point,
    pub width: usize,
    pub height: usize,
}

pub fn process_platform(
    q_entity: Query<(&Transform, Entity), Added<PlatformInstance>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/platform.png");

    for (transform, entity) in q_entity.iter() {
        commands
            .entity(entity)
            .despawn();

        let tween = Tween::new(
            EaseFunction::BounceOut,
            Duration::from_secs_f32(0.0),
            TransformPositionLens {
                start: Vec3::ONE,
                end: Vec3::ONE,
            },
        );

        let platform = commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                transform.translation.x,
                transform.translation.y,
                0.0,
            )),
            GravityScale(0.0),
            Collider::cuboid(12.0, 5.5),
            RigidBody::KinematicPositionBased,
            Platform {
                initial_pos: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                ..default()
            },
            Despawnable {},
            bevy_tweening::Animator::new(tween),
        )).id();

        let platform_texture = commands.spawn(
            SpriteBundle {
                transform: Transform::from_xyz(
                    0.0,
                    6.0,
                    0.0,
                ),
                texture: texture_handle.clone(),
                ..default()
            },
        ).id();

        commands.entity(platform).add_child(platform_texture);

    }
}

pub fn process_elevator(
    q_entity: Query<(&Transform, &Level, Entity), Added<ElevatorInstance>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/elevator.png");

    for (transform, level, entity) in q_entity.iter() {
        commands
            .entity(entity)
            .despawn();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                ),
                texture: texture_handle.clone(),
                ..default()
            },
            GravityScale(0.0),
            Collider::cuboid(16.0, 5.5),
            RigidBody::KinematicPositionBased,
            Despawnable {},
            Elevator {
                direction: Vec2::new(0.0, 0.3),
                initial_position: transform.translation.truncate(),
            },
            level.clone(),
        ));
    }
}

pub fn process_sharpener(
    q_entity: Query<(&Transform, &PointTo, Entity), Added<SharpenerInstance>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/pin.png");

    for (transform, point_to, entity) in q_entity.iter() {
        commands
            .entity(entity)
            .despawn();
        
        let tween = Tween::new(
            EaseFunction::SineInOut,
            Duration::from_secs_f32(3.0),
            TransformPositionLens {
                start: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                end: Vec3::new(point_to.x as f32, transform.translation.y, 0.0),
            }
        )
            .with_repeat_count(RepeatCount::Infinite)
            .with_repeat_strategy(bevy_tweening::RepeatStrategy::MirroredRepeat);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                ),
                texture: texture_handle.clone(),
                ..default()
            },
            Collider::cuboid(4.5, 4.5),
            Sensor,
            Despawnable {},
            Sharpener {},
            Interaction::default(),
            bevy_tweening::Animator::new(tween),
        ));
    }
}

pub fn process_pin(
    q_entity: Query<(&Transform, Entity), Added<PinInstance>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/pin.png");

    for (transform, entity) in q_entity.iter() {
        commands
            .entity(entity)
            .despawn();

        let tween = Tween::new(
            EaseFunction::SineInOut,
            Duration::from_secs_f32(2.0),
            TransformPositionLens {
                start: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                end: Vec3::new(transform.translation.x, transform.translation.y + 6.0, 0.0),
            }
        )
            .with_repeat_count(RepeatCount::Infinite)
            .with_repeat_strategy(bevy_tweening::RepeatStrategy::MirroredRepeat);

        let opacity_tween = Tween::new(
            EaseFunction::SineInOut,
            Duration::from_secs_f32(0.0),
            SpriteColorLens {
                start: Color::WHITE,
                end: Color::WHITE,
            }
        );

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                ),
                texture: texture_handle.clone(),
                ..default()
            },
            Collider::cuboid(4.5, 4.5),
            Sensor,
            Interaction::default(),
            Despawnable {},
            Pin {
                initial_position: Vec2::new(transform.translation.x, transform.translation.y),
                ..default()
            },
            bevy_tweening::Animator::new(tween),
            bevy_tweening::Animator::new(opacity_tween),
        ));
    }
}

pub fn process_spawn_point(
    mut q_entity: Query<(&mut Transform, Entity), Added<SpawnPoint>>,
    mut q_player: Query<(&mut Transform, &mut Player), Without<SpawnPoint>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: bevy::prelude::Handle<Image> = asset_server.load("sprites/pencil_box.png");

    for (mut transform, entity) in q_entity.iter_mut() {
        let player = q_player.get_single_mut();

        if player.is_err() {
            continue
        }

        let (mut player_transform, mut player) = player.unwrap();

        player_transform.translation.x = transform.translation.x;
        player_transform.translation.y = transform.translation.y;

        commands.entity(entity).insert((
            Sprite {
                ..default()
            },
            texture_handle.clone(),
        ));

        transform.translation.z = Z_INDEX_PENCIL_BOX;
        player.respawn_timer = Timer::from_seconds(PLAYER_HIT_RESPAWN_TIME, TimerMode::Once);
        player.is_respawning = true;
    }
}

pub fn process_exit (
    q_entity: Query<(&Transform, &RequiredKeys, Entity), Added<ExitInstance>>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ev_pin_pickup: EventWriter<PinUiUpdated>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/school_locker.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(36.0, 73.0),
        2,
        1,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (transform, required_keys, entity) in q_entity.iter() {
        commands
            .entity(entity)
            .despawn();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                ),
                ..default()
            },
            Collider::cuboid(12.0, 24.0),
            Sensor,
            RequiredKeys(required_keys.0),
            Exit::default(),
            Interaction::default(),
            Despawnable {},
        ));

        game_state.required_keys = required_keys.0 as isize;
        ev_pin_pickup.send(PinUiUpdated());
    }
}

pub fn setup_walls(
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
                x: (transform.translation.x / 24.0) as i32,
                y: (transform.translation.y / 24.0) as i32,
            }
        );
    }

    if points.is_empty() {
        return;
    }

    let points_with_neighbors = find_points_with_neighbors(&points);
    let neighbor_set: std::collections::HashSet<_> = points_with_neighbors.into_iter().collect();
    points.retain(|p| !neighbor_set.contains(p));

    let rects = find_rectangles(&points);

    for rect in rects.iter() {
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                rect.top_left.x as f32 * 24.0 + rect.width as f32 * 24.0 / 2.0,
                rect.top_left.y as f32 * 24.0 + 12.0, 0.0
            )),
            RigidBody::Fixed,
            Collider::cuboid(12.0 * rect.width as f32, 12.0),
            Despawnable {},
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

