use bevy::{prelude::{AssetServer, Commands, Res, NodeBundle, default, BuildChildren, ButtonBundle, Changed, Query, AudioBundle, PlaybackSettings, With, ResMut, GlobalVolume, AudioSink, AudioSinkPlayback, Entity}, ui::{Style, PositionType, Val, UiImage, UiRect, FlexDirection, JustifyContent, AlignItems, Interaction}, audio::{PlaybackMode, VolumeLevel}};
use kt_common::components::ui::{SoundUi, MuteButtonUi};

pub fn sound_ui (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let sound_ui_container = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                padding: UiRect::left(Val::Px(16.0)),
                ..default()
            },
            ..default()
        },
        SoundUi {}
    );

    commands.spawn(sound_ui_container).with_children(|parent| {
        parent.spawn(
            (ButtonBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(32.0),
                    margin: UiRect::left(Val::Px(9.0)),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/sound.png")),
                ..default()
            },
            MuteButtonUi {},
            )
        );
    });
}

pub fn handle_sound_button_interactions(
    q_sound_ui_container: Query<Entity, With<SoundUi>>,
    mut q_interaction: Query<(&Interaction, Entity), (Changed<Interaction>, With<MuteButtonUi>)>,
    mut global_volume: ResMut<GlobalVolume>,
    music_controller: Query<&AudioSink>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, entity) in &mut q_interaction {
        if let Interaction::Pressed = *interaction {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/btn_click.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    ..default()
                },
            });

            if global_volume.volume.get() == 0.0 {
                global_volume.volume = VolumeLevel::new(1.0);
                for sink in music_controller.iter() {
                    sink.play();
                }

                commands.entity(entity).remove_parent();
                commands.entity(entity).despawn();

                for container in q_sound_ui_container.iter() {
                    commands
                        .entity(container)
                        .with_children(|parent| {
                            parent.spawn(
                                (ButtonBundle {
                                    style: Style {
                                        width: Val::Px(30.0),
                                        height: Val::Px(32.0),
                                        margin: UiRect::left(Val::Px(9.0)),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load("sprites/sound.png")),
                                    ..default()
                                },
                                MuteButtonUi {},
                                )
                            );
                        });
                }

            } else {
                global_volume.volume = VolumeLevel::new(0.0);
                for sink in music_controller.iter() {
                    sink.pause();
                }

                commands.entity(entity).remove_parent();
                commands.entity(entity).despawn();

                for container in q_sound_ui_container.iter() {
                    commands
                        .entity(container)
                        .with_children(|parent| {
                            parent.spawn(
                                (ButtonBundle {
                                    style: Style {
                                        width: Val::Px(30.0),
                                        height: Val::Px(32.0),
                                        margin: UiRect::left(Val::Px(9.0)),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load("sprites/audio_disabled.png")),
                                    ..default()
                                },
                                MuteButtonUi {},
                                )
                            );
                        });
                }
            }
        }
    }
}
