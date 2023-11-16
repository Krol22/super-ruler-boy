use std::ops::Mul;

use bevy::{prelude::{Query, Res, Vec2}, time::Time};
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};
use kt_common::components::{velocity::Velocity, acceleration::Acceleration, gravity::GravityDir, jump::Jump};

pub fn apply_velocity_to_kinematic_controller(
    mut q_kinematic_controller: Query<(&mut KinematicCharacterController, &mut Velocity, &mut Acceleration, &GravityDir)>,
    time: Res<Time>,
) {
    for (mut kcc, mut velocity, mut acceleration, gravity_dir) in q_kinematic_controller.iter_mut() {
        // Apply gravity
        velocity.current += Vec2::new(0.0, -9.0 * gravity_dir.dir as f32);

        // Movement
        velocity.current += Vec2::new(
            acceleration.current.x,
            acceleration.current.y
        );

        velocity.current = velocity.current.clamp(-velocity.max, velocity.max);
        kcc.translation = Some(velocity.current.mul(time.delta_seconds()));

        // Damp velocity
        velocity.current = Vec2::new(
            velocity.current.x * (1.0 - velocity.damping),
            velocity.current.y * (1.0 - velocity.damping)
        );

        // Clear acceleration
        acceleration.current = Vec2::ZERO;
    }
}

pub fn clear_velocity_if_kinematic_on_ground(
    mut q_kinematic: Query<(&KinematicCharacterControllerOutput, &Jump, &mut Velocity)>,
) {
    for (kcco, jump, mut velocity) in q_kinematic.iter_mut() {
        if kcco.grounded && !jump.is_jumping {
            velocity.current.y = 0.0;
        }
    }
}
