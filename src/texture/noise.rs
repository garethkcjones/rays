use super::Texture;
use crate::{Colour, Perlin, Vector};

#[derive(Clone, Debug)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    #[must_use]
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: Vector) -> Colour {
        Colour::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * p))
    }
}
