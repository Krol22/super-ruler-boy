use bevy::{prelude::{Query, Res, Vec2}, time::Time};
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};
use kt_common::components::{velocity::Velocity, acceleration::Acceleration, gravity::GravityDir, jump::Jump};

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
    mut q_kinematic: Query<(&KinematicCharacterControllerOutput, &Jump, &mut Velocity)>,
) {
    for (kcco, jump, mut velocity) in q_kinematic.iter_mut() {
        if kcco.grounded && !jump.is_jumping {
            velocity.current.y = -40.0;
        }
    }
}
