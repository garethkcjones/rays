use super::Texture;
use crate::{Colour, Perlin, Vec3};
use std::sync::Arc;

/**
 * Type for representing a random noise texture.
 */
#[derive(Debug, Default)]
pub struct Noise {
    noise: Perlin,
}

impl Noise {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    #[must_use]
    pub fn new_texture() -> Arc<dyn Texture> {
        Arc::new(Self::new())
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Colour {
        Colour(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
