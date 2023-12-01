use bevy::{prelude::{Commands, Res, AssetServer, NodeBundle, default, BuildChildren, ChildBuilder, ImageBundle, EventReader, Query, Entity, With, Without}, ui::{Style, Val, FlexDirection, JustifyContent, Display, UiImage, AlignItems, UiRect}};
use bevy_persistent::Persistent;
use kt_common::{events::PinUiUpdated, components::ui::{PinsContainerUI, PinUI}};

use crate::save_game::GameState;

pub fn setup_in_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<Persistent<GameState>>,
) {
    let in_game_ui_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    };

    let top_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            padding: UiRect::right(Val::Px(16.0)),
            ..default()
        },
        ..default()
    };

    let pins_grid = (NodeBundle {
        style: Style {
            display: Display::Flex,
            ..default()
        },
        ..default()
    }, PinsContainerUI {});

    commands
        .spawn(in_game_ui_container)
        .with_children(|parent| {
            parent
                .spawn(top_container)
                .with_children(|top| {
                    top.spawn(pins_grid);
                });
        });
}

pub fn create_pin(grid: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    grid
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(27.0),
                    height: Val::Px(27.0),
                    margin: UiRect::left(Val::Px(9.0)),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/pin.png")),
                ..default()
            },
            PinUI {},
        ));
}

pub fn create_disabled_pin(grid: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    grid
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(27.0),
                    height: Val::Px(27.0),
                    margin: UiRect::left(Val::Px(9.0)),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/pin_disabled.png")),
                ..default()
            },
            PinUI {},
        ));
}

pub fn consume_pin_ui_update_events(
    mut ev_pin_ui_update: EventReader<PinUiUpdated>,
    mut q_pins_container: Query<Entity, (With<PinsContainerUI>, Without<PinUI>)>,
    mut q_pins: Query<Entity, (With<PinUI>, Without<PinsContainerUI>)>,
    mut commands: Commands,
    game_state: Res<Persistent<GameState>>,
    asset_server: Res<AssetServer>,
) {
    for _ev in ev_pin_ui_update.iter() {
        let disabled_keys = game_state.required_keys - game_state.picked_keys;

        for entity in q_pins_container.iter_mut() {
            for pin in q_pins.iter_mut() {
                commands.entity(pin).remove_parent();
                commands.entity(pin).despawn();
            }
            
            commands
                .entity(entity)
                .with_children(|parent| {
                    for _number in 1..= disabled_keys {
                        create_disabled_pin(parent, &asset_server);
                    }

                    for _number in 1..=game_state.picked_keys {
                        create_pin(parent, &asset_server);
                    }
                });
        }
    }
}
