use bevy::prelude::*;
use euclid::Trig;

use crate::physics::*;


#[derive(Component)]
struct Bullet;

#[derive(Event)]
pub struct BulletShootEvent {
    pub origin: Vec2,
    pub direction: Vec2,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullets)
            .add_event::<BulletShootEvent>();
    }
}

fn spawn_bullets(
    mut events: EventReader<BulletShootEvent>,
    mut commands: Commands,
) {
    for e in events.iter() {
        commands.spawn((
            Bullet,
            Velocity(e.direction.normalize_or_zero() * e.speed),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(2.0, 4.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::from((e.origin, 0.0)),
                    rotation:  Quat::from_rotation_z(Vec2::Y.angle_between(e.direction)),
                    ..default()
                },
                ..default()
            }
        ));
    }
}
