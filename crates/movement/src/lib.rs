use bevy::prelude::{Plugin, App, Update, IntoSystemConfigs};
use horizontal::{horizontal_controls, horizontal_controls_on_ceiling};
use jumping::{jumping_controls, update_can_jump_flag, bounce_off_ceiling};
use physics::{apply_velocity_to_kinematic_controller, clear_velocity_if_kinematic_on_ground, hit_ground};
use stretching::{stretching_controls, grab_ceiling, ungrab_ceiling};

mod jumping;
mod horizontal;
mod stretching;
mod physics;

#[derive(Debug, Default)]
pub struct MovementPlugin {}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            horizontal_controls,
            horizontal_controls_on_ceiling,
            jumping_controls,
            update_can_jump_flag,
            bounce_off_ceiling,
            stretching_controls,
            grab_ceiling,
            ungrab_ceiling,
            apply_velocity_to_kinematic_controller,
            clear_velocity_if_kinematic_on_ground,
            hit_ground,
        ).chain());
    }
}
