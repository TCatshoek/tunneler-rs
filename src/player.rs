use bevy::prelude::*;
use bevy::math::Vec2;
#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, move_player);
    }
}

pub fn setup_player(
    mut commands: Commands
) {
    let left_track = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(2.0, 14.0)),
            ..default()
        },
        transform: Transform::from_xyz(-6.0, 0.0, 0.0),
        ..default()
    }).id();

    let right_track = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(2.0, 14.0)),
            ..default()
        },
        transform: Transform::from_xyz(6.0, 0.0, 0.0),
        ..default()
    }).id();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player
    ))
        .push_children(&[
            left_track,
            right_track
        ]);
}

pub fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform) in query.iter_mut() {
        let mut direction = bevy::math::Vec2::splat(0.0);

        if keys.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            direction.x += 1.0
        }

        if let Some(direction) = direction.try_normalize() {
            transform.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(direction));
            transform.translation.x += direction.x;
            transform.translation.y += direction.y;
            println!("Player pos: {}", transform.translation)
        }
    };
}