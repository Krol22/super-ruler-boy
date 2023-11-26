use std::time::Duration;

use bevy::{prelude::{App, default, Commands, ResMut, Assets, Res, AssetServer, Vec2, SpatialBundle, Vec3, Transform, BuildChildren, Startup, Query, Children, With, Update, IntoSystemConfigs, KeyCode, Input, Rect, Without, Entity, Camera, ImagePlugin, Color, in_state, OnEnter, States, Component, Resource}, DefaultPlugins, window::{WindowPlugin, Window, WindowResolution, PresentMode}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite, SpriteBundle, Sprite}, utils::HashMap, time::{Time, Timer, TimerMode}, ecs::{schedule::ExecutorKind }, diagnostic::{FrameTimeDiagnosticsPlugin}, ui::{Style, Val, UiRect}, };
use bevy::prelude::PluginGroup;

use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection, prelude::{LdtkIntCellAppExt, LdtkEntityAppExt}, LdtkSettings, LevelBackground, LayerMetadata};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_rapier2d::prelude::{RigidBody, Collider, KinematicCharacterController, QueryFilterFlags, RapierContext, QueryFilter};
use bevy_tweening::{Tween, EaseFunction, lens::{TransformScaleLens, TransformPositionLens, SpriteColorLens, UiPositionLens}};
use kt_common::{CommonPlugin, components::{limb::{Limb, LimbType}, player::Player, jump::Jump, gravity::GravityDir, velocity::Velocity, acceleration::Acceleration, checkpoint::Checkpoint, ground_detector::GroundDetector, dust_particle_emitter::DustParticleEmitter, pin::{Pin, PinState}, interaction::{Interaction}, ui::{TransitionColumnLeftUi, TransitionColumnRightUi}, ldtk::{WallBundle, SpikesBundle, SpawnPointBundle, CheckpointBundle, ElevatorBundle, PlatformBundle, PinBundle, SharpenerBundle, SpawnPoint, Level, Elevator, HitComponent}}};
use kt_core::{CorePlugin, animation::{Animation, Animator, animator_sys}, particle::ParticleEmitter};
use kt_movement::MovementPlugin;
use kt_util::constants::{WINDOW_TITLE, INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT, PLAYER_HIT_RESPAWN_TIME, PLAYER_CAMERA_MARGIN_X, ASPECT_RATIO_X, ASPECT_RATIO_Y, PLAYER_CAMERA_MARGIN_Y};
use bevy_save::{prelude::*, WorldSaveableExt};
use main_menu_ui::{setup_menu, handle_play_button_interactions, handle_level_button_interactions, handle_back_button_interactions};
use process_ldtk_world::{process_spawn_point, process_elevator, process_platform, process_pin, process_sharpener, setup_walls};
use save_game::{GameState, load};
use screen_transitions::{complete_transition_event_handler, setup_transition_ui, switch_levels_transition_event_handler, save_game_after_transition};

pub mod save_game;
pub mod main_menu_ui;
pub mod screen_transitions;
pub mod process_ldtk_world;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
              title: WINDOW_TITLE.to_string(),
              resizable: false,
              resolution: WindowResolution::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT),
              present_mode: PresentMode::AutoVsync,
              ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(FramepacePlugin)
        .add_plugins(LdtkPlugin)
        .add_plugins(SavePlugins)
        .add_plugins(CommonPlugin {})
        .add_plugins(CorePlugin {})
        .add_plugins(MovementPlugin {})
        // .add_plugins(ParallaxPlugin {})
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(60.0)
        })
        .edit_schedule(Update, |schedule| {
            schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        });

    app
        .add_state::<AppState>();

    app
        .insert_resource(GameState::default())
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LevelDimensions::default())
        .insert_resource(LdtkSettings {
            level_background: LevelBackground::Nonexistent,
            ..default()
        })
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_entity::<SpikesBundle>("Spikes")
        .register_ldtk_entity::<SpawnPointBundle>("SpawnPoint")
        .register_ldtk_entity::<CheckpointBundle>("Checkpoint")
        .register_ldtk_entity::<ElevatorBundle>("Elevator")
        .register_ldtk_entity::<PlatformBundle>("Platform")
        .register_ldtk_entity::<PinBundle>("Pin")
        .register_ldtk_entity::<SharpenerBundle>("Sharpener")
        .register_saveable::<GameState>();

/*
    GLOBAL
*/
    app.add_systems(Startup, load);

/*
   MENU STATE
*/
    app
        .add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(Update, handle_play_button_interactions.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, handle_level_button_interactions.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, handle_back_button_interactions.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, complete_transition_event_handler.run_if(in_state(AppState::MainMenu)));

/*
    IN GAME STATE
*/
    app
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnEnter(AppState::InGame), spawn_player)
        .add_systems(Startup, setup_transition_ui)
        .add_systems(Update, setup_walls.run_if(in_state(AppState::InGame)))
        .add_systems(Update, process_spawn_point.run_if(in_state(AppState::InGame)))
        .add_systems(Update, process_elevator.run_if(in_state(AppState::InGame)))
        .add_systems(Update, process_platform.run_if(in_state(AppState::InGame)))
        .add_systems(Update, process_pin.run_if(in_state(AppState::InGame)))
        .add_systems(Update, process_sharpener.run_if(in_state(AppState::InGame)))
        .add_systems(Update, pickup_pin.run_if(in_state(AppState::InGame)))
        .add_systems(Update, switch_levels_transition_event_handler.run_if(in_state(AppState::InGame)))
        .add_systems(Update, save_game_after_transition.run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
    reset_overlaps,
    handle_player_interaction,
    restart_player_pos,
    handle_animation,
    animator_sys,
    handle_extension_stretch,
    handle_stretching,
    flip_depend_on_velocity,
    handle_player_hurt_collision,
    handle_activate_checkpoint,
    checkpoint_sprites_handle,
    respawn_player,
    update_level_dimensions,
    follow_player_with_camera,
    elevator_handle,
    handle_pin,
    load_next_level,
).chain()
    .run_if(in_state(AppState::InGame)));
    
    app.run();
}

fn load_next_level(
    input: Res<Input<KeyCode>>,
    mut q_transition_left: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnLeftUi>, Without<TransitionColumnRightUi>)>,
    mut q_transition_right: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnRightUi>, Without<TransitionColumnLeftUi>)>,
) {
    if input.just_pressed(KeyCode::N) {
        let mut transition_left_column_animator = q_transition_left.single_mut();
        let mut transition_right_column_animator = q_transition_right.single_mut();

        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs_f32(0.5),
            UiPositionLens {
                start: UiRect {
                    left: Val::Percent(100.0),
                    top: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                end: UiRect {
                    left: Val::Percent(40.0),
                    top: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
            },
        ).with_completed_event(1);

        transition_left_column_animator.set_tweenable(tween);

        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs_f32(0.5),
            UiPositionLens {
                start: UiRect {
                    right: Val::Percent(100.0),
                    top: Val::Auto,
                    left: Val::Auto,
                    bottom: Val::Auto,
                },
                end: UiRect {
                    right: Val::Percent(40.0),
                    top: Val::Auto,
                    left: Val::Auto,
                    bottom: Val::Auto,
                },
            },
        );

        transition_right_column_animator.set_tweenable(tween);
    }
}

fn restart_player_pos(
    q_spawn_point: Query<&Transform, With<SpawnPoint>>,
    mut q_player: Query<&mut Transform, (With<Player>, Without<SpawnPoint>)>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::R) {
        for transform in q_spawn_point.iter() {
            let player = q_player.get_single_mut();

            if player.is_err() {
                continue
            }

            let mut player_transform = player.unwrap();

            player_transform.translation.x = transform.translation.x;
            player_transform.translation.y = transform.translation.y;
        }
    }
}

fn elevator_handle(
    mut q_elevator: Query<(&mut Transform, &mut Elevator, &Level)>
) {
    for (mut transform, mut elevator, level) in q_elevator.iter_mut() {
        if level.0 == 0 {
            continue;
        }

        transform.translation.x += elevator.direction.x;
        transform.translation.y += elevator.direction.y;

        if level.0 < 0 {
            if transform.translation.y > elevator.initial_position.y {
                elevator.direction.y = -elevator.direction.y
            }

            if transform.translation.y < elevator.initial_position.y + level.0 as f32 * 24.0 {
                elevator.direction.y = -elevator.direction.y
            }

            continue;
        }

        if level.0 > 0 {
            if transform.translation.y > elevator.initial_position.y + level.0 as f32 * 24.0 {
                elevator.direction.y = -elevator.direction.y
            }

            if transform.translation.y < elevator.initial_position.y {
                elevator.direction.y = -elevator.direction.y
            }
        }

    }
}

fn handle_pin(
    mut q_pin: Query<(
        &mut Pin,
        &Transform,
        &mut bevy_tweening::Animator<Transform>,
        &mut bevy_tweening::Animator<Sprite>,
    )>,
) {
    for (mut pin, transform, mut transform_animator, mut sprite_animator) in q_pin.iter_mut() {
        if !pin.picked && matches!(pin.state.current, PinState::Picked) && !pin.state.is_same_as_previous() {

            let tween_up = Tween::new(
                EaseFunction::SineInOut,
                Duration::from_secs_f64(0.3),
                TransformPositionLens {
                    start: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                    end: Vec3::new(transform.translation.x, transform.translation.y + 24.0, 0.0),
                }
            );

            let opacity = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f64(0.3),
                SpriteColorLens {
                    start: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 },
                    end: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 },
                }
            );

            transform_animator.set_tweenable(tween_up);
            sprite_animator.set_tweenable(opacity);

            pin.picked = true;
        }
    }
}

fn pickup_pin(
    mut q_pins: Query<(&mut Pin, &Interaction)>,
) {
    for (mut pin, interaction) in q_pins.iter_mut() {
        if interaction.is_overlapping && !pin.picked {
            pin.state.update_value(PinState::Picked);
        }
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct LevelDimensions {
    pub width: f32,
    pub height: f32,
}

fn update_level_dimensions (
    q_layers: Query<&LayerMetadata>,
    mut level_dimensions: ResMut<LevelDimensions>,
) {
    for layer in q_layers.iter() {
        if layer.identifier != "Tiles" {
            continue;
        }

        let width = layer.c_wid * layer.grid_size;
        let height = layer.c_hei * layer.grid_size;

        level_dimensions.width = width as f32;
        level_dimensions.height = height as f32;
    }
}

pub fn follow_player_with_camera(
    q_player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut q_camera: Query<(&mut Transform, Entity), With<Camera>>,
    // mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    level_dimensions: Res<LevelDimensions>,
) {
    let player = if let Ok(player) = q_player.get_single() {
        player
    } else {
        return
    };

    let (mut camera, entity) = if let Ok(camera) = q_camera.get_single_mut() {
        camera
    } else {
        return
    };

    let x_margin = ASPECT_RATIO_X * ((PLAYER_CAMERA_MARGIN_X as f32 / 2.0) / 100.0);
    let y_margin = ASPECT_RATIO_Y * ((PLAYER_CAMERA_MARGIN_Y as f32 / 2.0) / 100.0);

    let left_edge = camera.translation.x - x_margin;
    let right_edge = camera.translation.x + x_margin;

    let top_edge = camera.translation.y + y_margin;
    let bottom_edge = camera.translation.y - y_margin;

    let mut speed = Vec2::ZERO;
    let mut new_pos_x = camera.translation.x;

    if player.translation.x < left_edge {
        new_pos_x = camera.translation.x - (left_edge - player.translation.x);
        speed.x = -1.0;
    }

    if player.translation.x > right_edge {
        new_pos_x = camera.translation.x + player.translation.x - right_edge;
        speed.x = 1.0;
    }

    if player.translation.x - ASPECT_RATIO_X / 2.0 + x_margin <= 0.0 {
        new_pos_x = ASPECT_RATIO_X / 2.0;
        speed.x = 0.0;
    }

    if player.translation.x + ASPECT_RATIO_X / 2.0 - x_margin >= level_dimensions.width {
        new_pos_x = level_dimensions.width - ASPECT_RATIO_X / 2.0;
        speed.x = 0.0;
    }

    let mut new_pos_y = camera.translation.y;

    if player.translation.y > top_edge {
        new_pos_y = camera.translation.y + (player.translation.y - top_edge);
        speed.y = 1.0;
    }

    if player.translation.y < bottom_edge {
        new_pos_y = camera.translation.y - (bottom_edge - player.translation.y);
        speed.y = -1.0;
    }

    if player.translation.y - ASPECT_RATIO_Y / 2.0 + y_margin <= 0.0 {
        new_pos_y = ASPECT_RATIO_Y / 2.0;
        speed.y = 0.0;
    }

    if player.translation.y + ASPECT_RATIO_Y / 2.0 - y_margin >= level_dimensions.height {
        new_pos_y = level_dimensions.height - ASPECT_RATIO_Y / 2.0;
        speed.y = 0.0;
    }

    camera.translation.x = new_pos_x;
    camera.translation.y = new_pos_y;
    // speed.x = new_pos_x - camera.translation.x;
    // speed.y = new_pos_y - camera.translation.y;

    // move_event_writer.send(ParallaxMoveEvent {
        // camera_move_speed: speed,
        // camera: entity,
    // });
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
    q_checkpoint: Query<&Transform, (With<SpawnPoint>, Without<Player>)>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in q_player.iter_mut() {
        player.respawn_timer.tick(time.delta());

        if player.respawn_timer.just_finished() {
            for spawnpoint_transform in q_checkpoint.iter() {
                transform.translation.x = spawnpoint_transform.translation.x;
                transform.translation.y = spawnpoint_transform.translation.y + 5.0;
            }
        }

        if player.respawn_timer.finished() {
            player.is_respawning = false;
        }
    }

}

fn handle_player_interaction (
    q_player: Query<(&mut Transform, &Velocity), With<Player>>,
    mut q_interaction: Query<&mut Interaction>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (transform, velocity) in q_player.iter() {
        let shape = Collider::cuboid(6.0, 9.0);
        let shape_pos = transform.translation.truncate();
        let shape_vel = Vec2::new(
            velocity.current.x * time.delta_seconds(),
            velocity.current.y * time.delta_seconds(),
        );
        let shape_rot = 0.0;
        let max_toi = 1.0;

        let filter = QueryFilter {
            flags: QueryFilterFlags::EXCLUDE_SOLIDS,
            ..default()
        };

        if let Some((entity, _hit)) = rapier_context.cast_shape(
            shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
        ) {
            let interaction = q_interaction.get_mut(entity);
            if let Ok(mut interaction) = interaction {
                interaction.is_overlapping = true;
            }

            continue
        }
    }
}

fn reset_overlaps (
    mut q_interaction: Query<&mut Interaction>,
) {
    for mut interaction in q_interaction.iter_mut() {
        interaction.is_overlapping = false;
    }
}

fn handle_player_hurt_collision(
    mut q_player: Query<(&mut Transform, &Velocity, &mut Player)>,
    q_hit: Query<&HitComponent>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (transform, velocity, mut player) in q_player.iter_mut() {
        if !player.respawn_timer.finished() && !player.respawn_timer.paused() {
            continue;
        }

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
            let hit_component = q_hit.get(entity);

            if hit_component.is_ok() {
                player.respawn_timer = Timer::from_seconds(PLAYER_HIT_RESPAWN_TIME, TimerMode::Once);
                player.is_respawning = true;
            }
            continue
        }
    }
}

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

fn create_transform_tween(from_x: f32, to_x: f32) -> Tween<Transform> {
    Tween::new(
        EaseFunction::BounceOut,
        Duration::from_secs_f32(0.2),
        TransformScaleLens {
            start: Vec3::new(from_x, 1.0, 1.0),
            end: Vec3::new(to_x, 1.0, 1.0),
        }
    )
}

fn handle_animation(
    q_player: Query<(&Velocity, &Children), With<Player>>,    
    mut q_player_limbs: Query<(&Children, &mut bevy_tweening::Animator<Transform>, &Transform), With<PlayerLimbs>>,
    mut q_limbs: Query<&mut Animator, With<Limb>>,
) {
    for (velocity, children) in q_player.iter() {
        let mut animation_name = "Idle".to_string();

        if velocity.current.x.abs() > 8.0 {
            animation_name = "Move".to_string();
        }

        for &child in children.iter() {
            let child = q_player_limbs.get_mut(child);

            let (player_limbs, mut animator, transform) = match child {
                Ok(child) => child,
                Err(..) => continue,
            };

            if velocity.current.x < 0.0 && transform.scale.x >= 1.0 {
                animator.set_tweenable(create_transform_tween(1.0, -1.0));
            }

            if velocity.current.x > 0.0 && transform.scale.x <= -1.0 {
                animator.set_tweenable(create_transform_tween(-1.0, 1.0));
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

    let tween = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_secs_f32(0.2),
        TransformScaleLens {
            start: Vec3::ONE,
            end: Vec3::ONE,
        },
    );

    let player = commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(50.0, 200.0, 0.0)),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(6.0, 9.0),
        GroundDetector::default(),
        GravityDir {
            dir: 0.0,
            slow_down: 1.0,
        },
        Velocity {
            damping: 0.1,
            max: Vec2::new(80.0, 300.0),
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
        bevy_tweening::Animator::new(tween),
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
        fps: 12,
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
        fps: 12,
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
        fps: 12,
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

    let dust_handle = asset_server.load("sprites/dust-sheet-copy.png");
    let dust_texture_atlas = TextureAtlas::from_grid(
        dust_handle,
        Vec2::new(24.0, 24.0),
        4,
        1,
        None,
        None,
    );

    let dust_texture_atlas = texture_atlases.add(dust_texture_atlas);

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 2.0),
        ParticleEmitter {
            frames: vec![0, 1, 2, 3],
            particle_lifetime: 0.4,
            handle: dust_texture_atlas,
            spawning: false,
            spawn_timer: Timer::from_seconds(0.4, TimerMode::Once),
        },
        DustParticleEmitter {},
    ));
}
