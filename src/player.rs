use bevy::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::KeyCode::ArrowUp;
use euclid::Trig;
use crate::bullet::BulletShootEvent;
use crate::physics::quat_to_2d_rotation;
use crate::pixelcollider::PixelCollider;

#[derive(Component, Deref, DerefMut)]
pub struct OGTransform(Transform);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, move_player)
            .add_systems(Update, fire_bullets);
    }
}

pub fn setup_player(
    mut commands: Commands
) {

    let body_width = 6.0;
    let body_length = 8.0;
    let track_length = 10.0;
    let track_width = 2.0;
    let barrel_length = 8.0;
    let barrel_width = 2.0;

    let left_track = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(track_width, track_length)),
            ..default()
        },
        transform: Transform::from_xyz(-body_width / 2.0 - 1.0, 0.0, 0.0),
        ..default()
    }).id();

    let right_track = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(track_width, track_length)),
            ..default()
        },
        transform: Transform::from_xyz(body_width / 2.0 + 1.0, 0.0, 0.0),
        ..default()
    }).id();

    let barrel = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.75, 0.75, 0.1),
            custom_size: Some(Vec2::new(barrel_width, barrel_length)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, barrel_length / 2.0, 0.01),
        ..default()
    }).id();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(body_width, body_length)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PixelCollider::new(),
        Player,
        OGTransform(Transform::from_xyz(0.0, 0.0, 0.0))
    ))
        .push_children(&[
            left_track,
            right_track,
            barrel
        ]);
}

pub fn fire_bullets(
    mut query: Query<&Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<BulletShootEvent>
) {
    for transform in query.iter() {

        let (mut axis, mut angle) = transform.rotation.to_axis_angle();

        if axis.z < 0.0 {
            axis = -axis;
            angle = -angle;
        }

        if keys.just_pressed(KeyCode::Space) {

            let direction = Vec2::from_angle(angle).rotate(Vec2::Y);
            let spawnpoint_offset = Vec2::new(10.0, 0.0).rotate(direction);
            let spawnpoint = transform.translation.truncate() + spawnpoint_offset;


            events.send(BulletShootEvent{
                origin: spawnpoint,
                direction,
                speed: 1000.0,
            });
        }
    }
}
pub fn move_player(
    mut query: Query<(&mut Transform, &mut OGTransform), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dtime = time.delta_seconds();
    let movement_speed = 20.0;

    for (mut transform, mut og_transform) in query.iter_mut() {

        let mut direction = bevy::math::Vec2::splat(0.0);

        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0
        }

        if let Some(direction) = direction.try_normalize() {
            let desired_rotation = Quat::from_rotation_z(Vec2::Y.angle_between(direction));

            transform.rotation = transform.rotation.lerp(desired_rotation, dtime * movement_speed);

            transform.translation.x = og_transform.translation.x;
            transform.translation.y = og_transform.translation.y;

            transform.translation.x += direction.x * dtime * movement_speed;
            transform.translation.y += direction.y * dtime * movement_speed;

            og_transform.translation.x = transform.translation.x;
            og_transform.translation.y = transform.translation.y;

            transform.translation.x = f32::round(transform.translation.x);
            transform.translation.y = f32::round(transform.translation.y);
        }
    };
}