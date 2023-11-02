use std::process::Command;
use bevy::ecs::query::WorldQuery;
use rand::{random, Rng};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const SIZE: (u32, u32) = (512, 512);

#[derive(Component)]
pub struct Map {
    image_handle: Handle<Image>
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_map)
            .add_systems(Update, update_map);
    }
}

pub fn init_map(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>
) {
    let img = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );

    let img_handle = images.add(img);

    commands.spawn((
        Map {
            image_handle: img_handle.clone()
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
                ..default()
            },
            texture: img_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        }
    ));
}

pub fn update_map(
    query: Query<&Map>,
    mut images: ResMut<Assets<Image>>
) {
    for map in query.iter() {
        let img_handle = &map.image_handle;
        if let Some(image) = images.get_mut(img_handle) {
            image.data.fill(random())
        }
    }
}