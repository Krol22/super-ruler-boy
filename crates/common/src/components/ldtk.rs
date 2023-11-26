use bevy::{prelude::{Bundle, Component, Vec2, default}, sprite::SpriteSheetBundle, reflect::Reflect};
use bevy_ecs_ldtk::{LdtkIntCell, LdtkEntity, EntityInstance, IntGridCell, prelude::LdtkFields};
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor};

use super::checkpoint::Checkpoint;

#[derive(Default, Bundle, LdtkEntity)]
pub struct CheckpointBundle {
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub checkpoint: Checkpoint,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpikesBundle {
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub hit_component: HitComponent,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpawnPointBundle {
    pub spawn_point: SpawnPoint,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct ElevatorBundle {
    #[with(Level::from_field)]
    pub level: Level,
    pub elevator: ElevatorInstance,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlatformBundle {
    pub platform_instance: PlatformInstance,
}

#[derive(Default, Component, Clone, Debug)]
pub struct PlatformInstance {}

#[derive(Default, Bundle, LdtkEntity)]
pub struct ExitBundle {
    pub exit_instance: ExitInstance,
    #[with(RequiredKeys::from_field)]
    pub required_keys: RequiredKeys,
}

#[derive(Default, Component, Clone, Debug)]
pub struct ExitInstance {}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PinBundle {
    pub pin_instance: PinInstance,
}

#[derive(Default, Component, Clone, Debug)]
pub struct PinInstance {}

#[derive(Clone, Component, Debug, Default)]
pub struct SpawnPoint {}

#[derive(Clone, Component, Debug, Default)]
pub struct WallDefinition {}


#[derive(Clone, Component, Debug, Default)]
pub struct HitComponent {}

#[derive(Default, Bundle, LdtkEntity)]
pub struct SharpenerBundle {
    #[with(PointTo::from_field)]
    pub point_to: PointTo,
    pub sharpener_instance: SharpenerInstance,
}

#[derive(Default, Component, Clone, Debug)]
pub struct SharpenerInstance {}

#[derive(Clone, Component, Debug, Default, Reflect, PartialEq, PartialOrd)]
pub struct RequiredKeys(pub i32);

impl RequiredKeys {
    pub fn from_field(entity_instance: &EntityInstance) -> RequiredKeys {
        RequiredKeys(*entity_instance
            .get_int_field("required_pins")
            .expect("expected entity to have non-nullable required_pins int field"))
    }
}

#[derive(Clone, Component, Debug, Default, Reflect, PartialEq, PartialOrd)]
pub struct Level(pub i32);

impl Level {
    pub fn from_field(entity_instance: &EntityInstance) -> Level {
        Level(*entity_instance
            .get_int_field("level")
            .expect("expected entity to have non-nullable level string field"))
    }
}

#[derive(Clone, Component, Debug, Default, Reflect, PartialEq, PartialOrd)]
pub struct PointTo {
    pub x: i32,
    pub y: i32,
}


impl PointTo {
    pub fn from_field(entity_instance: &EntityInstance) -> PointTo {
        let point_field = *entity_instance
            .get_point_field("point_to")
            .expect("expeced entity to have non-nullable point_to point field");

        PointTo {
            x: point_field.x,
            y: point_field.y,
        }
    }
}

#[derive(Clone, Component, Default, Debug)]
pub struct ElevatorInstance {}

#[derive(Clone, Component, Default, Debug)]
pub struct Elevator {
    pub initial_position: Vec2,
    pub direction: Vec2,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    #[from_int_grid_cell]
    pub collider_bundle: ColliderBundle,
    pub wall: WallDefinition,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        if int_grid_cell.value == 1 {
            ColliderBundle {
                collider: Collider::cuboid(12., 12.),
                rigid_body: RigidBody::Fixed,
            }
        } else {
            ColliderBundle::default()
        }
    }
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(_entity_instance: &EntityInstance) -> ColliderBundle {
        ColliderBundle {
            collider: Collider::cuboid(2., 2.),
            rigid_body: RigidBody::Fixed,
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
}

impl From<&EntityInstance> for SensorBundle {
    fn from(entity_instance: &EntityInstance) -> SensorBundle {
        match entity_instance.identifier.as_ref() {
            "Spikes" => SensorBundle {
                collider: Collider::cuboid(10.0, 3.0),
                ..default()
            },
            "Checkpoint" => SensorBundle {
                collider: Collider::cuboid(8.0, 8.0),
                ..default()
            },
            "Stapler" => SensorBundle {
                collider: Collider::cuboid(12.0, 12.0),
                ..default()
            },
            _ => SensorBundle::default()
        }
    }
}

