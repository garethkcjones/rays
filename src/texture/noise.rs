use super::Texture;
use crate::{Colour, Perlin, Vec3};
use std::sync::Arc;

/**
 * Type for representing a random noise texture.
 */
#[derive(Debug)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    #[must_use]
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Default::default(),
            scale,
        }
    }

    #[must_use]
    pub fn new_texture(scale: f64) -> Arc<dyn Texture> {
        Arc::new(Self::new(scale))
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Colour {
        Colour(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * p))
    }
}
