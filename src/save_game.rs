use bevy::{prelude::{Resource, ReflectResource, World}, reflect::Reflect};
use bevy_save::WorldSaveableExt;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameState {
    pub unlocked_levels: isize,
    pub current_level: isize,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            unlocked_levels: 1,
            current_level: 1,
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
}

// fn save(world: &World) {
    // let keys = world.resource::<Input<KeyCode>>();

    // if keys.just_released(KeyCode::Return) {
        // world.save("gol").expect("Failed to save");
    // }
// }
