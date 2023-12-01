use std::{time::Duration};

use bevy::{prelude::{Component, ChildBuilder, AssetServer, Res, ButtonBundle, Color, TextBundle, default, BuildChildren, Query, Changed, Without, With, NodeBundle, Commands, ResMut, ImageBundle, AudioBundle, PlaybackSettings, AudioSink, AudioSinkPlayback}, ui::{Style, Val, UiRect, JustifyContent, AlignItems, BackgroundColor, UiImage, PositionType, FlexDirection, ZIndex, GridTrack, Display}, text::TextStyle, audio::PlaybackMode};
use bevy_ecs_ldtk::LevelSelection;
use bevy_tweening::{Tween, EaseFunction, lens::UiPositionLens};
use kt_common::components::{ui::{PlayButtonUi, MainColumnUi, LevelSelectColumnUi, LevelSelectButtonUi, TransitionColumnLeftUi, TransitionColumnRightUi, ButtonClickSound}, despawnable::Despawnable};

use crate::save_game::GameState;

pub fn create_play_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    height: Val::Px(65.0),
                    width: Val::Px(350.0),
                    margin: UiRect::all(Val::Px(8.0)),
                    padding: UiRect::horizontal(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                image: UiImage::new(asset_server.load("sprites/button.png")),
                ..default()
            },
            PlayButtonUi {}
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "start game",
                TextStyle {
                    font: asset_server.load("fonts/ThaleahFat.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            ));
        });
}

pub fn handle_play_button_interactions(
    mut q_interaction: Query<&bevy::ui::Interaction, (Changed<bevy::ui::Interaction>, With<PlayButtonUi>)>,
    mut q_main_column: Query<&mut Style, (With<MainColumnUi>, Without<LevelSelectColumnUi>)>,
    mut q_level_select_column: Query<&mut Style, (With<LevelSelectColumnUi>, Without<MainColumnUi>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for interaction in &mut q_interaction {
        if let bevy::ui::Interaction::Pressed = *interaction {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/btn_click.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    ..default()
                },
            });

            let mut main_column_style = q_main_column.single_mut();
            let mut level_select_column_style = q_level_select_column.single_mut();

            main_column_style.left = Val::Percent(100.0);
            level_select_column_style.right = Val::Auto;
        }
    }
}

#[derive(Clone, Component, Debug, Default)]
pub struct BackButtonUi {}

pub fn create_back_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    height: Val::Px(65.0),
                    width: Val::Px(350.0),
                    margin: UiRect::all(Val::Px(8.0)),
                    padding: UiRect::horizontal(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                image: UiImage::new(asset_server.load("sprites/button.png")),
                ..default()
            },
            BackButtonUi {}
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "back",
                TextStyle {
                    font: asset_server.load("fonts/ThaleahFat.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            ));
        });
}

pub fn handle_back_button_interactions(
    mut q_interaction: Query<&bevy::ui::Interaction, (Changed<bevy::ui::Interaction>, With<BackButtonUi>)>,
    mut q_main_column: Query<&mut Style, (With<MainColumnUi>, Without<LevelSelectColumnUi>)>,
    mut q_level_select_column: Query<&mut Style, (With<LevelSelectColumnUi>, Without<MainColumnUi>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for interaction in &mut q_interaction {
        if let bevy::ui::Interaction::Pressed = *interaction {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/btn_click.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    ..default()
                },
            });

            let mut main_column_style = q_main_column.single_mut();
            let mut level_select_column_style = q_level_select_column.single_mut();

            level_select_column_style.right = Val::Percent(100.0);
            main_column_style.left = Val::Auto;
        }
    }
}

pub fn create_level_button(grid: &mut ChildBuilder, number: isize, asset_server: &Res<AssetServer>) {
    let level = format!("{:02}", number);

    grid
        .spawn(NodeBundle {
            style: Style::default(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(65.0),
                            width: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(8.0)),
                            padding: UiRect::horizontal(Val::Px(6.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        image: UiImage::new(asset_server.load("sprites/small_btn.png")),
                        ..default()
                    },
                    LevelSelectButtonUi { level: number },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        level,
                        TextStyle {
                            font: asset_server.load("fonts/ThaleahFat.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },
                    ));
                });
        });
}

pub fn create_disabled_level_button(grid: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    grid
        .spawn(NodeBundle {
            style: Style::default(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(65.0),
                            width: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(8.0)),
                            padding: UiRect::horizontal(Val::Px(6.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        image: UiImage::new(asset_server.load("sprites/small_btn_disabled.png")),
                        ..default()
                    },
                ));
        });
}

pub fn handle_level_button_interactions(
    mut q_interaction: Query<(&bevy::ui::Interaction, &LevelSelectButtonUi), Changed<bevy::ui::Interaction>>,
    mut q_transition_left: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnLeftUi>, Without<TransitionColumnRightUi>)>,
    mut q_transition_right: Query<&mut bevy_tweening::Animator<Style>, (With<TransitionColumnRightUi>, Without<TransitionColumnLeftUi>)>,
    mut game_state: ResMut<GameState>,
    mut level_selection: ResMut<LevelSelection>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, level_select_button) in &mut q_interaction {
        if let bevy::ui::Interaction::Pressed = *interaction {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/SFX_powerUp10.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    ..default()
                },
            });

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
            ).with_completed_event(3);

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
            game_state.current_level = level_select_button.level;
            dbg!(level_select_button.level);
            *level_selection = LevelSelection::Index(level_select_button.level as usize - 1);
        }
    }
}

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    let main_menu_ui_container = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        Despawnable {},
    );

    let main_menu_background = (
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            image: UiImage::new(asset_server.load("sprites/menu_background.png")),
            ..default()
        }
    );

    let logo = (
        ImageBundle {
            style: Style {
                width: Val::Px(115.0 * 4.0),
                height: Val::Px(82.0 * 4.0),
                margin: UiRect::bottom(Val::Px(64.0)),
                ..default()
            },
            image: UiImage::new(asset_server.load("sprites/logo.png")),
            ..default()
        }
    );

    let main_column = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            z_index: ZIndex::Global(1),
            ..default()
        },
        MainColumnUi {},
    );

    let level_select_column = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                right: Val::Percent(100.0),
                ..default()
            },
            z_index: ZIndex::Global(1),
            ..default()
        },
        LevelSelectColumnUi {},
    );

    let grid_container = NodeBundle {
        style: Style {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0)],
            margin: UiRect::vertical(Val::Px(32.0)),
            ..default()
        },
        ..default()
    };

    let unlocked_levels = game_state.unlocked_levels;
    let locked_levels = 12 - game_state.unlocked_levels;

    commands
        .spawn(main_menu_ui_container)
        .with_children(|parent| {
            parent.spawn(main_menu_background);

            parent
                .spawn(main_column)
                .with_children(|parent| {
                    parent.spawn(logo);

                    create_play_button(parent, &asset_server);
                });

            parent
                .spawn(level_select_column)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Choose level",
                        TextStyle {
                            font: asset_server.load("fonts/ThaleahFat.ttf"),
                            font_size: 62.0,
                            color: Color::rgb(0.96, 0.67, 0.1),
                        }
                    ));

                    parent
                        .spawn(grid_container)
                        .with_children(|grid| {
                            for number in 1..=unlocked_levels {
                                create_level_button(grid, number, &asset_server);
                            }
                            for _ in 1..=locked_levels {
                                create_disabled_level_button(grid, &asset_server);
                            }
                        });

                    create_back_button(parent, &asset_server);
                });
        });

}
