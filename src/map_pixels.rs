use std::process::Command;
use bevy::ecs::query::WorldQuery;
use rand::{random, Rng};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use crate::ImageWriter::ImageWriter;
use crate::mouse::MouseWorldPosition;
use crate::pixelcollider::PixelCollider;

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
                flip_y: true,
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
    pixelcolliders: Query<&PixelCollider>,
    mut images: ResMut<Assets<Image>>,
    mut events: EventReader<MouseWorldPosition>
) {


    let mut x = 0;
    let mut y = 0;
    for pos in events.read() {
        println!("world mouse pos: ({}, {})", pos.x, pos.y);
        x = (SIZE.0 as f32 / 2.0 + pos.x) as u32;
        y = (SIZE.1 as f32 / 2.0 + pos.y) as u32;
        println!("mouse pos ({}, {})", x, y);
    }

    if let Ok(map) = query.get_single() {
        let img_handle = &map.image_handle;
        if let Some(image) = images.get_mut(img_handle) {
            image.data.fill(0xFF);
            let mut writer = ImageWriter(image);
            for pixelcollider in pixelcolliders.iter() {
                for (x, y) in &pixelcollider.pixels {
                    let x = (x + (SIZE.0 / 2) as i32) as u32;
                    let y = (y + (SIZE.1 / 2) as i32) as u32;
                    println!("Drawing at: ({}, {})", x, y);
                    writer.put_pixel(x, y, Color::RED);
                }
            }

        }
    }
}