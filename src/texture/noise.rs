#![allow(clippy::new_ret_no_self)]

use super::Texture;
use crate::{Colour, Perlin, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    #[must_use]
    pub fn new(scale: f64) -> Arc<dyn Texture> {
        Arc::new(Self {
            noise: Perlin::new(),
            scale,
        })
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: Vector) -> Colour {
        // Colour::new(1.0, 1.0, 1.0) * self.noise.turb(self.scale * p, 7)
        Colour::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
