use super::Texture;
use crate::{Colour, Vec3};
use std::sync::Arc;

/**
 * Type for representing a chequered texture.
 */
#[derive(Debug)]
pub struct Chequer {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Chequer {
    #[must_use]
    pub fn new(even: impl Into<Arc<dyn Texture>>, odd: impl Into<Arc<dyn Texture>>) -> Self {
        Self {
            even: even.into(),
            odd: odd.into(),
        }
    }

    #[must_use]
    pub fn new_texture(
        even: impl Into<Arc<dyn Texture>>,
        odd: impl Into<Arc<dyn Texture>>,
    ) -> Arc<dyn Texture> {
        Arc::new(Self::new(even, odd))
    }
}

impl Texture for Chequer {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
