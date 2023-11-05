use bevy::{prelude::{Plugin, Resource, Vec2, Query, With, GlobalTransform, Camera, ResMut, App, Update}, window::{Window, PrimaryWindow}};

#[derive(Resource, Default)]
pub struct MousePosition {
    pub position: Vec2,
    pub world_pos: Vec2,
}

fn set_mouse_position_resource(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_position.world_pos = Vec2::new(world_position.x, world_position.y);
    }

}

#[derive(Debug, Default)]
pub struct MousePlugin {}

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MousePosition>()
            .add_systems(Update, set_mouse_position_resource);
    }
}
