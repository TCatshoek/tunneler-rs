
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub(crate) struct Velocity(pub Vec2);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn quat_to_2d_rotation(quat: Quat) -> f32 {
    let m_0_0 = 1.0 - (2.0 * quat.y * quat.y) - (2.0 * quat.z * quat.z);
    let m_1_0 = (2.0 * quat.x * quat.y) + (2.0 * quat.w * quat.z);
    let angle = f32::atan2(m_1_0, m_0_0);
    angle
}