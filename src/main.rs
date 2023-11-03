mod player;
mod map;
mod bullet;
mod physics;
mod mouse;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use player::*;
use map::*;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::bullet::BulletPlugin;
use crate::physics::PhysicsPlugin;
use crate::mouse::MousePositionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(MousePositionPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, camera_control)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn camera_control(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>
) {
    let dist = time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if keys.pressed(KeyCode::PageUp) {
            log_scale -= dist;
        }
        if keys.pressed(KeyCode::PageDown) {
            log_scale += dist;
        }

        projection.scale = log_scale.exp();
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}