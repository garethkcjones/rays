use super::Texture;
use crate::{Colour, Vec3};
use std::sync::Arc;

/**
 * Type for representing a chequered texture.
 */
#[derive(Debug)]
pub struct Chequer {
    scale: Vec3,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Chequer {
    #[must_use]
    pub fn new(
        scale: Vec3,
        even: impl Into<Arc<dyn Texture>>,
        odd: impl Into<Arc<dyn Texture>>,
    ) -> Self {
        Self {
            scale,
            even: even.into(),
            odd: odd.into(),
        }
    }

    #[must_use]
    pub fn new_texture(
        scale: Vec3,
        even: impl Into<Arc<dyn Texture>>,
        odd: impl Into<Arc<dyn Texture>>,
    ) -> Arc<dyn Texture> {
        Arc::new(Self::new(scale, even, odd))
    }
}

impl Texture for Chequer {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour {
        let p = p * self.scale;
        let sines = p.x().sin() * p.y().sin() * p.z().sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
