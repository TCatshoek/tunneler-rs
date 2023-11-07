use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use image::Rgba;

pub struct ImageWriter<'a>(pub &'a mut Image);

impl<'a> ImageWriter<'a> {
    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        let width = self.0.texture_descriptor.size.width;
        let height = self.0.texture_descriptor.size.height;

        if x >= width || y >= height {
            return;
        }

        match self.0.texture_descriptor.format {
            TextureFormat::Rgba8Unorm => self.put_pixel_rgba8unorm(x, y, color),
            _ => panic!("Unsupported texture format")
        }
    }

    fn put_pixel_rgba8unorm(&mut self, x: u32, y: u32, color: Color) {
        // bytes per pixel
        const bpp: usize = 4;

        let Extent3d {width, height, .. } = self.0.texture_descriptor.size;

        let index: usize = (x * bpp as u32 + y * width * bpp as u32) as usize;

        let mut pixel: &mut [u8; bpp] = (&mut self.0.data[index..index + bpp]).try_into().unwrap();

        *pixel = color.as_rgba_u8();
    }
}