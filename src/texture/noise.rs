use super::Texture;
use crate::{Colour, Perlin, Vector};

#[derive(Clone, Debug, Default)]
pub struct Noise {
    noise: Perlin,
}

impl Noise {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: Vector) -> Colour {
        Colour::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
