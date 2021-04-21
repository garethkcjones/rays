use super::Texture;
use crate::{Colour, Vec3};
use image::{io::Reader, RgbImage};
use std::{error::Error, path::Path, sync::Arc};

/**
 * Type for image textures.
 */
#[derive(Debug)]
pub struct Image {
    data: RgbImage,
}

impl Image {
    pub fn new(filename: impl AsRef<Path>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let data = match Reader::open(filename.as_ref()) {
            Ok(data) => data,
            Err(x) => {
                return Err(format!(
                    "cannot open texture image file “{}”: {}",
                    filename.as_ref().display(),
                    x
                )
                .into())
            }
        };

        let data = match data.decode() {
            Ok(data) => data,
            Err(x) => {
                return Err(format!(
                    "cannot decode texture image file “{}”: {}",
                    filename.as_ref().display(),
                    x
                )
                .into())
            }
        };

        let data = data.to_rgb8();

        Ok(Self { data })
    }

    pub fn new_texture(
        filename: impl AsRef<Path>,
    ) -> Result<Arc<dyn Texture>, Box<dyn Error + Send + Sync>> {
        Ok(Arc::new(Self::new(filename)?))
    }
}

impl Texture for Image {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Colour {
        #![allow(clippy::many_single_char_names)]

        let width = self.data.width();
        let height = self.data.height();

        // Clamp input texture coordinates to [0, 1] x [1, 0].
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates.

        let i = (u * f64::from(width)) as u32;
        let j = (v * f64::from(height)) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0.
        let i = i.min(width - 1);
        let j = j.min(height - 1);

        const COLOUR_SCALE: f64 = 1.0 / 255.0;

        let pixel = self.data.get_pixel(i, j).0;
        let r = f64::from(pixel[0]);
        let g = f64::from(pixel[1]);
        let b = f64::from(pixel[2]);
        let pixel = Colour(r, g, b);

        COLOUR_SCALE * pixel
    }
}
