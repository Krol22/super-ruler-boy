use bevy::{prelude::{Query, Res, Vec2, With}, time::{Time, TimerMode, Timer}};
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};
use kt_common::components::{velocity::Velocity, acceleration::Acceleration, gravity::GravityDir, jump::Jump, ground_detector::{GroundDetector, self}, dust_particle_emitter::DustParticleEmitter};
use kt_core::particle::ParticleEmitter;

pub fn apply_velocity_to_kinematic_controller(
    mut q_kinematic_controller: Query<(&mut KinematicCharacterController, &mut Velocity, &mut Acceleration, &GravityDir)>,
) {
    for (mut kcc, mut velocity, mut acceleration, gravity_dir) in q_kinematic_controller.iter_mut() {
        // Apply gravity
        velocity.current += Vec2::new(0.0, -15.0 * gravity_dir.dir * gravity_dir.slow_down);

        // Movement
        velocity.current += Vec2::new(
            acceleration.current.x,
            acceleration.current.y
        );

        velocity.current = velocity.current.clamp(-velocity.max, velocity.max);
        // dbg!(time.delta_seconds());
        kcc.translation = Some(
            Vec2::new(
                velocity.current.x * (1.0 / 60.0),
                velocity.current.y * (1.0 / 60.0),
            )
        );

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
    mut q_kinematic: Query<(&KinematicCharacterControllerOutput, &Jump, &mut Velocity, &mut GroundDetector)>,
) {
    for (kcco, jump, mut velocity, mut ground_detector) in q_kinematic.iter_mut() {
        let original = velocity.current.y;
        if kcco.grounded && !jump.is_jumping {
            velocity.current.y = -40.0;
        }

        ground_detector.is_on_ground.update_value(kcco.grounded);
        if !ground_detector.is_on_ground.is_same_as_previous() {
            ground_detector.hit_speed = original;
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
