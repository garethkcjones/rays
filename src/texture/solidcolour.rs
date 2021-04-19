use super::Texture;
use crate::{Colour, Vec3};
use std::sync::Arc;

/**
 * Type for representing solid colour textures.
 */
#[derive(Debug)]
pub struct SolidColour {
    value: Colour,
}

impl SolidColour {
    #[must_use]
    pub fn new(value: Colour) -> Self {
        Self { value }
    }

    #[must_use]
    pub fn new_texture(value: Colour) -> Arc<dyn Texture> {
        Arc::new(Self::new(value))
    }
}

impl From<Colour> for Arc<dyn Texture> {
    fn from(colour: Colour) -> Self {
        SolidColour::new_texture(colour)
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Colour {
        self.value
    }
}
