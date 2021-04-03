#![allow(clippy::new_ret_no_self)]

use super::Texture;
use crate::{Colour, Vector};
use image::{io::Reader as ImageReader, RgbImage};
use std::{path::Path, sync::Arc};

#[derive(Debug)]
pub struct OpaqueImage {
    data: RgbImage,
}

impl OpaqueImage {
    #[must_use]
    pub fn new(filename: impl AsRef<Path>) -> Arc<dyn Texture> {
        let data = ImageReader::open(filename).expect("Cannot open image texture file");
        let data = data.decode().expect("Cannot decode image texture");
        let data = data.to_rgb8();

        Arc::new(Self { data })
    }
}

impl Texture for OpaqueImage {
    fn value(&self, u: f64, v: f64, _p: Vector) -> Colour {
        let width = self.data.width();
        let height = self.data.height();

        // Clamp input texture coordinates to [0, 1] x [1, 0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let x = (u * f64::from(width)) as u32;
        let y = (v * f64::from(height)) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        let x = x.min(width - 1);
        let y = y.min(height - 1);

        const COLOUR_SCALE: f64 = 1.0 / 255.0;
        let pixel = self.data.get_pixel(x, y).0;
        let r = COLOUR_SCALE * f64::from(pixel[0]);
        let g = COLOUR_SCALE * f64::from(pixel[1]);
        let b = COLOUR_SCALE * f64::from(pixel[2]);

        Colour::new(r, g, b)
    }
}
