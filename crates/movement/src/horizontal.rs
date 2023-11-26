use bevy::prelude::{Query, Transform, Res, Input, KeyCode, Vec2, default};
use bevy_rapier2d::prelude::{RapierContext, Collider, QueryFilter, QueryFilterFlags};
use kt_common::components::{velocity::Velocity, player::Player};
use kt_util::constants::PLAYER_HORIZONTAL_MOVE_SPEED;

pub fn horizontal_controls (
    mut q_player: Query<(&mut Velocity, &Player, &Transform)>,
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut velocity, player, transform) in q_player.iter_mut() {
        if player.grabbed_ceiling {
            continue;
        }

        if player.is_respawning {
            continue;
        }

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            let shape = Collider::cuboid(6.0, 9.0 + player.stretch / 2.0);
            let shape_pos = transform.translation.truncate() + Vec2::new(-0.2, player.stretch / 2.0 + 0.1);
            let shape_vel = Vec2::new(-1.0, 0.0);
            let shape_rot = 0.0;
            let max_toi = 1.0;

            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS, 
                ..default()
            };

            if let Some(_entity) = rapier_context.cast_shape(
                shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
            ) {
                velocity.current.x = 0.0;
                continue
            }

            velocity.current.x = -PLAYER_HORIZONTAL_MOVE_SPEED;
        } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            let shape = Collider::cuboid(6.0, 9.0 + player.stretch / 2.0);
            let shape_pos = transform.translation.truncate() + Vec2::new(0.2, player.stretch / 2.0 + 0.1);
            let shape_vel = Vec2::new(1.0, 0.0);
            let shape_rot = 0.0;
            let max_toi = 1.0;

            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS, 
                ..default()
            };

            if let Some(_entity) = rapier_context.cast_shape(
                shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
            ) {
                velocity.current.x = 0.0;
                continue
            }

            velocity.current.x = PLAYER_HORIZONTAL_MOVE_SPEED;
        }

    }
}

pub fn horizontal_controls_on_ceiling (
    mut q_player: Query<(&mut Velocity, &mut Player, &Transform)>,
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player = q_player.get_single_mut();
    let (mut velocity, mut player, transform) = match player {
        Ok(player) => player,
        Err(..) => return,
    };

    if !player.grabbed_ceiling {
        return;
    }

    velocity.current.x = 0.0;

    let shape = Collider::cuboid(6.0, 9.0);
    let shape_pos = transform.translation.truncate();
    let shape_vel = Vec2::new(0.0, player.stretch + 2.0);
    let shape_rot = 0.0;
    let max_toi = 1.0;
    let filter = QueryFilter {
        flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS, 
        ..default()
    };

    if let Some(_entity) = rapier_context.cast_shape(
        shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
    ) {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.current.x = -100.0;
        } 

        if keyboard_input.pressed(KeyCode::Right) {
            velocity.current.x = 100.0;
        } 
    } else {
        player.grabbed_ceiling = false;
    }
}

