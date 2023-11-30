use std::time::Duration;

use bevy::{prelude::{Query, Res, Vec2, With, Transform, Vec3, Without}, time::{Time, TimerMode, Timer}};
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_tweening::{EaseFunction, lens::TransformPositionLens, Tween};
use kt_common::components::{velocity::Velocity, acceleration::Acceleration, gravity::GravityDir, jump::Jump, ground_detector::{GroundDetector}, dust_particle_emitter::DustParticleEmitter, platform::Platform, player::Player};
use kt_core::particle::ParticleEmitter;

pub fn apply_velocity_to_kinematic_controller(
    mut q_kinematic_controller: Query<(&mut KinematicCharacterController, &mut Velocity, &mut Acceleration, &GravityDir)>,
) {
    for (mut kcc, mut velocity, mut acceleration, gravity_dir) in q_kinematic_controller.iter_mut() {
        // Apply gravity
        if velocity.current.y < 0.0 {
            velocity.current += Vec2::new(0.0, -15.0 * gravity_dir.dir * gravity_dir.slow_down);
        } else {
            velocity.current += Vec2::new(0.0, -15.0 * gravity_dir.dir);
        }

        // Movement
        velocity.current += Vec2::new(
            acceleration.current.x,
            acceleration.current.y
        );

        velocity.current = velocity.current.clamp(-velocity.max, velocity.max);
        if kcc.translation.is_none() {
            kcc.translation = Some(
                Vec2::new(
                    velocity.current.x * (1.0 / 60.0),
                    velocity.current.y * (1.0 / 60.0),
                )
            );
        }


        // Damp velocity
        velocity.current.x *= 1.0 - velocity.damping;

        if velocity.current.x.abs() < 0.1 {
            velocity.current.x = 0.0;
        }

        // Clear acceleration
        acceleration.current = Vec2::ZERO;
    }
}

pub fn clear_velocity_if_kinematic_on_ground(
    mut q_kinematic: Query<(&KinematicCharacterControllerOutput, &Jump, &mut Velocity, &mut GroundDetector, &Player)>,
) {
    for (kcco, jump, mut velocity, mut ground_detector, player) in q_kinematic.iter_mut() {
        let original = velocity.current.y;
        if kcco.grounded && !jump.is_jumping && !player.is_respawning {
            velocity.current.y = -40.0;
        }

        ground_detector.is_on_ground.update_value(kcco.grounded);
        if !ground_detector.is_on_ground.is_same_as_previous() {
            ground_detector.hit_speed = original;
        }
    }
}

pub fn activate_platforms(
    q_kinematic: Query<&KinematicCharacterControllerOutput>,
    mut q_platforms: Query<(&mut Platform, &Transform, &mut bevy_tweening::Animator<Transform>)>,
) {
    for kcco in q_kinematic.iter() {
        for collision in kcco.collisions.iter() {
            let platform = q_platforms.get_mut(collision.entity);

            if platform.is_err() {
                continue;
            }

            let (mut platform, transform, mut animator) = platform.unwrap();

            if platform.is_stepped_on {
                continue;
            }

            let tween = Tween::new(
                EaseFunction::BounceInOut,
                Duration::from_secs_f32(0.5),
                TransformPositionLens {
                    start: Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z),
                    end: Vec3::new(transform.translation.x, transform.translation.y - 5.0, transform.translation.z),
                }
            );

            animator.set_tweenable(tween);

            platform.is_stepped_on = true;
            platform.drop_timer = Timer::from_seconds(1.0, TimerMode::Once);
        }
    } 
}

pub fn handle_platform_dropping(
    mut q_platforms: Query<(&mut Platform, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut platform, mut transform) in q_platforms.iter_mut() {
        platform.drop_timer.tick(time.delta());

        if platform.drop_timer.finished() && platform.is_stepped_on {
            transform.translation.y -= 3.0;
        }
    }
}

pub fn handle_platform_off_screen(
    mut q_platforms: Query<(&mut Platform, &Transform, &mut bevy_tweening::Animator<Transform>)>,
    time: Res<Time>,
) {
    for (mut platform, transform, mut animator) in q_platforms.iter_mut() {
        if transform.translation.y > -30.0 {
            continue;
        }

        if platform.is_stepped_on {
            platform.restart_timer = Timer::from_seconds(1.0, TimerMode::Once);
        }

        platform.is_stepped_on = false;
        platform.restart_timer.tick(time.delta());

        if platform.restart_timer.just_finished() {
            let tween = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_secs_f32(1.5),
                TransformPositionLens {
                    start: Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z),
                    end: Vec3::new(platform.initial_pos.x, platform.initial_pos.y, platform.initial_pos.z),
                }
            );

            animator.set_tweenable(tween);
        }
    }
}

pub fn hit_ground(
    q_ground_detector: Query<&GroundDetector>,
    mut q_dust_emitter: Query<&mut ParticleEmitter, With<DustParticleEmitter>>
) {
    for ground_detector in q_ground_detector.iter() {
        if ground_detector.is_on_ground.current && !ground_detector.is_on_ground.is_same_as_previous() {
            for mut emitter in q_dust_emitter.iter_mut() {
                if ground_detector.hit_speed > -200.0 {
                    continue;
                }

                emitter.spawning = true;
                emitter.spawn_timer = Timer::from_seconds(0.01, TimerMode::Once);
            } 
        }
    }
}

pub fn sync_emitter_position(
    q_player: Query<&Transform, (With<Player>, Without<DustParticleEmitter>)>,
    mut q_particle_emitter: Query<&mut Transform, With<DustParticleEmitter>>,
) {
    for transform in q_player.iter() {
        for mut emitter_transform in q_particle_emitter.iter_mut() {
            emitter_transform.translation.x = transform.translation.x;
            emitter_transform.translation.y = transform.translation.y + 3.0;
        }
    }
}

