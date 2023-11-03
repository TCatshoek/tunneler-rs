use bevy::prelude::*;
use bevy::window::PrimaryWindow;


#[derive(Event)]
pub struct MouseWorldPosition(Vec2);

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cursor_position)
            .add_event::<MouseWorldPosition>();
    }
}

fn update_cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut events: EventWriter<MouseWorldPosition>
) {
    let (camera, camera_transform) = q_camera.single();

    if let Some(position) = q_windows.single().cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        events.send(MouseWorldPosition(position))
    }
}