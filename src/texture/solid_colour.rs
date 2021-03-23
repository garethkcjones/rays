use super::Texture;
use crate::{Colour, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct SolidColour {
    colour: Colour,
}

impl SolidColour {
    #[must_use]
    pub fn new(colour: Colour) -> Arc<Self> {
        Arc::new(Self { colour })
    }

    #[must_use]
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Arc<Self> {
        Self::new(Colour::new(red, green, blue))
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Vector) -> Colour {
        self.colour
    }
}
