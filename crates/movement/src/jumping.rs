use bevy::{prelude::{Query, Res, Input, KeyCode, Transform, Vec2, default, With}, time::{Time, Timer}};
use bevy_rapier2d::prelude::{KinematicCharacterControllerOutput, RapierContext, Collider, QueryFilter, QueryFilterFlags, KinematicCharacterController};
use kt_common::components::{velocity::Velocity, jump::Jump, player::Player};
use kt_util::constants::{PLAYER_JUMP_SPEED, JUMP_HOLD_FORCE};

pub fn jumping_controls (
    mut q_player: Query<(&mut Velocity, &mut Jump)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut velocity, mut jump) in q_player.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::X) {
            if !jump.can_jump {
                continue;
            }

            velocity.current.y = PLAYER_JUMP_SPEED;
            jump.is_jumping = true;
            jump.jump_timer = Timer::from_seconds(1.0, bevy::time::TimerMode::Once);
        }

        if keyboard_input.pressed(KeyCode::X) && jump.is_jumping && !jump.jump_timer.finished() {
            jump.jump_timer.tick(time.delta());
            velocity.current.y += JUMP_HOLD_FORCE * jump.jump_timer.percent_left();
        }

        if keyboard_input.just_released(KeyCode::X) || jump.jump_timer.finished() {
            jump.is_jumping = false;
        }
    }
}

pub fn update_can_jump_flag(
    mut q_player: Query<(&mut Jump, &KinematicCharacterControllerOutput)>,
) {
    for (mut jump, kcc_output) in q_player.iter_mut() {
        jump.can_jump = kcc_output.grounded;
    }
}

pub fn bounce_off_ceiling(
    mut q_player: Query<(&mut Velocity, &mut Jump, &Transform, &Player), With<KinematicCharacterController>>,
    rapier_context: Res<RapierContext>,
) {
    for (mut velocity, mut jump, transform, player) in q_player.iter_mut() {
        if player.grabbed_ceiling {
            continue;
        }

        let shape = Collider::cuboid(6.0, 9.0);
        let shape_pos = transform.translation.truncate();
        let shape_vel = Vec2::new(0.0, 2.0);
        let shape_rot = 0.0;
        let max_toi = 1.0;
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS, 
            ..default()
        };

        if let Some(_entity) = rapier_context.cast_shape(
            shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
        ) {
            velocity.current.y = -10.0;
            jump.is_jumping = false;
            continue;
        }
    }
}



