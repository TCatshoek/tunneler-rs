use bevy::prelude::*;
use bevy::render::primitives::Aabb;

#[derive(Component)]
pub struct PixelCollider {
    pub(crate) pixels: Vec<(i32, i32)>
}

impl PixelCollider {
    pub(crate) fn new() -> Self {
        return Self {
            pixels: Vec::new()
        }
    }
}

pub struct PixelColliderPlugin;

impl Plugin for PixelColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_pixel_colliders);
    }
}

fn recurse(
    id: Entity,
    aabbs: &Query<(&Aabb, &GlobalTransform)>,
    pixelcollider: &mut PixelCollider,
    children: &Query<&Children>
) {
    if let Ok((aabb, tf)) = aabbs.get(id) {
        determine_pixels(aabb, tf, &mut pixelcollider.pixels);
    }
    if let Ok(cur_children) = children.get(id) {
        for child in cur_children {
            recurse(*child, &aabbs, pixelcollider, &children);
        }
    }
}

fn determine_pixels(
    aabb: &Aabb,
    global_transform: &GlobalTransform,
    pixels: &mut Vec<(i32, i32)>
) {
    let transform = global_transform.compute_transform();
    let mut top_left = Vec3::new(aabb.center.x - aabb.half_extents.x, aabb.center.y + aabb.half_extents.y, 0.0);
    top_left = transform.rotation.mul_vec3( top_left);
    top_left = transform.translation + top_left;


    pixels.push((top_left.x as i32, top_left.y as i32));
}

pub fn update_pixel_colliders(
    mut entities: Query<(Entity, &mut PixelCollider)>,
    aabbs: Query<(&Aabb, &GlobalTransform)>,
    children: Query<&Children>
) {
    for (id, pixelcollider) in entities.iter_mut() {
        let pixels = pixelcollider.into_inner();
        pixels.pixels.clear();
        recurse(id, &aabbs, pixels, &children);
    }
}

