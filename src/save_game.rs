use bevy::{prelude::{Resource, ReflectResource, World, Entity, Commands, Query, With, GlobalTransform, Transform, Visibility, ResMut}, reflect::Reflect};
use bevy_save::WorldSaveableExt;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameState {
    pub unlocked_levels: isize,
    pub current_level: isize,
    pub picked_keys: isize,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            unlocked_levels: 1,
            current_level: 1,
            picked_keys: 0,
        }
    }
}

impl GameState {
    pub fn update_unlocked_levels(&mut self) {
        if self.current_level > self.unlocked_levels {
            self.unlocked_levels = self.current_level;
        }
    }
}

pub fn load(
    world: &mut World,
) {
    let _ = world.load("gol");
    dbg!("LOAD");
}

pub fn clean_entities(
    q_entities: Query<Entity, (With<GlobalTransform>, With<Transform>, With<Visibility>)>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
) {
    game_state.current_level = 0;
    game_state.picked_keys = 0;

    for entity in q_entities.iter() {
        commands.entity(entity).despawn();
    }
}
