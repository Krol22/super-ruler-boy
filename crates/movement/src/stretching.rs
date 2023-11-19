use bevy::prelude::{Query, Transform, Res, Input, KeyCode, Vec2, default};
use bevy_rapier2d::prelude::{RapierContext, QueryFilter, QueryFilterFlags};
use kt_common::components::{player::Player, jump::Jump, gravity::GravityDir, velocity::Velocity};
use kt_util::constants::{PLAYER_MAXIMUM_STRETCH, PLAYER_STRETCH_SPEED};

pub fn stretching_controls(
    mut q_player: Query<(&Transform, &mut Player, &mut Jump, &mut GravityDir)>,
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (transform, mut player, mut jump, mut gravity_dir) in q_player.iter_mut() {
        if player.grabbed_ceiling {
            continue;
        }

        if player.is_respawning {
            continue;
        }

        gravity_dir.slow_down = 1.0;
        if keyboard_input.pressed(KeyCode::Space) {

            if player.stretch >= PLAYER_MAXIMUM_STRETCH {
                player.stretch = PLAYER_MAXIMUM_STRETCH;
                continue;
            }

            let ray_pos = transform.translation.truncate();
            let ray_dir = Vec2::new(-1.1, player.stretch / 3.0 - 0.3);
            let max_toi = 4.0;
            let solid = true;
            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS, 
                ..default()
            };

            player.grabbed_ceiling = false;
            if let Some(_entity) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            ) {
                jump.is_jumping = false;
                player.grabbed_ceiling = true;
                continue;
            }

            let ray_pos = transform.translation.truncate();
            let ray_dir = Vec2::new(1.1, player.stretch / 3.0 - 0.3);
            let max_toi = 4.0;
            let solid = true;
            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS, 
                ..default()
            };

            if let Some(_entity) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            ) {
                jump.is_jumping = false;
                player.grabbed_ceiling = true;
                continue;
            }

            player.stretch += PLAYER_STRETCH_SPEED;
            gravity_dir.slow_down = -0.2;
            continue;
        }

        player.stretch -= PLAYER_STRETCH_SPEED;

        if player.stretch < 0.0 {
            player.stretch = 0.0;
        }
    }
}

pub fn ungrab_ceiling(
    mut q_player: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Z) {
        for mut player in q_player.iter_mut() {
            player.grabbed_ceiling = false;
        }
    }
}

pub fn grab_ceiling(
    mut q_player: Query<(&mut Player, &mut GravityDir, &mut Velocity)>,
) {
    for (mut player, mut gravity_dir, mut velocity) in q_player.iter_mut() {
        if player.grabbed_ceiling {
            gravity_dir.dir = -7.9;
            player.stretch -= (PLAYER_STRETCH_SPEED * 1.0);
            velocity.current.y = 0.0;
        } else {
            gravity_dir.dir = 1.0;
        }

        if player.stretch <= 0.0 {
            player.stretch = 0.0;
        }
    }
}

