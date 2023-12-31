use bevy::prelude::Component;

#[derive(Clone, Component, Debug, Default)]
pub struct PlayButtonUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct LevelSelectButtonUi {
    pub level: isize,
}

#[derive(Clone, Component, Debug, Default)]
pub struct ButtonClickSound {}

#[derive(Clone, Component, Debug, Default)]
pub struct MainMenuUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct MainColumnUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct SoundUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct MuteButtonUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct LevelSelectColumnUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct TransitionColumnLeftUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct TransitionColumnRightUi {}

#[derive(Clone, Component, Debug, Default)]
pub struct PinsContainerUI {}

#[derive(Clone, Component, Debug, Default)]
pub struct PinUI {}
