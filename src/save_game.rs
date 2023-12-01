use bevy::{prelude::{Resource, ReflectResource, World, Entity, Commands, Query, With, GlobalTransform, Transform, Visibility, ResMut}, reflect::Reflect};

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameState {
    pub unlocked_levels: isize,
    pub current_level: isize,
    pub picked_keys: isize,
    pub required_keys: isize,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            unlocked_levels: 1,
            current_level: 1,
            picked_keys: 0,
            required_keys: 1,
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

// pub fn load(
    // world: &mut World,
// ) {
    // let mut preexisting_entities = Vec::new();
    
    // // First, collect all entity IDs that existed before loading new ones.
    // for entity in world.iter_entities() {
        // preexisting_entities.push(entity.id());
    // }
    
    // // Perform the loading operation here.
    // let _ = world.load("gol");
    
    // // Collect IDs of entities to despawn
    // let mut to_despawn = Vec::new();
    
    // for entity in world.iter_entities() {
        // if !preexisting_entities.contains(&entity.id()) {
            // to_despawn.push(entity.id());
        // }
    // }
    
    // // Despawn entities in a separate step, after all borrows of `world` are completed.
    // for id in to_despawn {
        // world.despawn(id);
    // }
// }

// pub fn reset_state(
    // mut game_state: ResMut<GameState>,
// ) {
    // game_state.current_level = 0;
    // game_state.picked_keys = 0;
// }
