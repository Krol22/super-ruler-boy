use std::time::Duration;

use bevy::{prelude::{EventReader, Commands, Query, Entity, With, ResMut, Without, NextState, Res, AssetServer, NodeBundle, default, Color, BuildChildren, World}, ui::{Style, UiRect, Val, PositionType, FlexDirection, JustifyContent, BackgroundColor, ZIndex}};
use bevy_ecs_ldtk::LevelSelection;
use bevy_save::WorldSaveableExt;
use bevy_tweening::{TweenCompleted, Tween, EaseFunction, lens::UiPositionLens, EaseMethod, Delay};
use kt_common::components::{despawnable::Despawnable, ui::{TransitionColumnLeftUi, TransitionColumnRightUi}};

use crate::{AppState, save_game::GameState};

// Main Menu -> In Game
pub fn complete_transition_event_handler(
    mut q_event: EventReader<TweenCompleted>,
    mut commands: Commands,
    q_despawnable: Query<Entity, With<Despawnable>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut q_transition_left: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnLeftUi>, Without<TransitionColumnRightUi>)>,
    mut q_transition_right: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnRightUi>, Without<TransitionColumnLeftUi>)>,
) {
    for _event in q_event.iter() {
        next_state.set(AppState::InGame);

        for despawnable_entity in q_despawnable.iter() {
            commands.entity(despawnable_entity).despawn();
        }

        let mut transition_left_column_animator = q_transition_left.single_mut();
        let mut transition_right_column_animator = q_transition_right.single_mut();

        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs_f32(0.5),
            UiPositionLens {
                start: UiRect {
                    left: Val::Percent(40.0),
                    top: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                end: UiRect {
                    left: Val::Percent(100.0),
                    top: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
            },
        );

        transition_left_column_animator.set_tweenable(tween);

        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs_f32(0.5),
            UiPositionLens {
                start: UiRect {
                    right: Val::Percent(40.0),
                    top: Val::Auto,
                    left: Val::Auto,
                    bottom: Val::Auto,
                },
                end: UiRect {
                    right: Val::Percent(100.0),
                    top: Val::Auto,
                    left: Val::Auto,
                    bottom: Val::Auto,
                },
            },
        );

        transition_right_column_animator.set_tweenable(tween);
    }
}

// In game between levels
pub fn switch_levels_transition_event_handler (
    mut q_event: EventReader<TweenCompleted>,
    mut commands: Commands,
    q_despawnable: Query<Entity, With<Despawnable>>,
    mut q_transition_left: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnLeftUi>, Without<TransitionColumnRightUi>)>,
    mut q_transition_right: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnRightUi>, Without<TransitionColumnLeftUi>)>,
    mut level_selection: ResMut<LevelSelection>,
    mut game_state: ResMut<GameState>,
    // world: &World,
) {
    for event in q_event.iter() {
        if (event.user_data != 1) {
            continue;
        }

        *level_selection = LevelSelection::Index(game_state.current_level as usize);
        game_state.current_level += 1;

        game_state.update_unlocked_levels();
        // let _ = world.save("gol");

        for despawnable_entity in q_despawnable.iter() {
            commands.entity(despawnable_entity).despawn();
        }

        let mut transition_left_column_animator = q_transition_left.single_mut();
        let mut transition_right_column_animator = q_transition_right.single_mut();

        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs_f32(0.5),
            UiPositionLens {
                start: UiRect {
                    left: Val::Percent(40.0),
                    top: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                end: UiRect {
                    left: Val::Percent(100.0),
                    top: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
            },
        );

        transition_left_column_animator.set_tweenable(tween);

        let tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_secs_f32(0.5),
            UiPositionLens {
                start: UiRect {
                    right: Val::Percent(40.0),
                    top: Val::Auto,
                    left: Val::Auto,
                    bottom: Val::Auto,
                },
                end: UiRect {
                    right: Val::Percent(100.0),
                    top: Val::Auto,
                    left: Val::Auto,
                    bottom: Val::Auto,
                },
            },
        ).with_completed_event(2);

        transition_right_column_animator.set_tweenable(tween);
    }
}

pub fn save_game_after_transition (
    mut q_event: EventReader<TweenCompleted>,
    world: &World, 
) {
    for event in q_event.iter() {
        if event.user_data != 2 {
            continue;
        }

        let _ = world.save("gol");
    }
}

pub fn setup_transition_ui(
   mut commands: Commands,
) {
    let ui_container = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
    );

    let tween = Tween::new(
        EaseMethod::Linear,
        Duration::from_secs(0),
        UiPositionLens {
            start: UiRect::DEFAULT,
            end: UiRect::DEFAULT,
        },
    );

    let transition_column_left = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(100.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            z_index: ZIndex::Global(99),
            ..default()
        },
        TransitionColumnLeftUi {},
        bevy_tweening::Animator::new(Delay::new(Duration::from_secs_f32(1.0)).then(tween)),
    );

    let tween = Tween::new(
        EaseMethod::Linear,
        Duration::from_secs(0),
        UiPositionLens {
            start: UiRect::DEFAULT,
            end: UiRect::DEFAULT,
        },
    );

    let transition_column_right = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(100.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            z_index: ZIndex::Global(99),
            ..default()
        },
        TransitionColumnRightUi {},
        bevy_tweening::Animator::new(Delay::new(Duration::from_secs_f32(1.0)).then(tween)),
    );

    commands
        .spawn(ui_container)
        .with_children(|parent| {
            parent
                .spawn(transition_column_left);

            parent
                .spawn(transition_column_right);
        });
}
