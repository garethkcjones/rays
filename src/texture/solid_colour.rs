#![allow(clippy::new_ret_no_self)]

use super::Texture;
use crate::{Colour, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct SolidColour {
    colour: Colour,
}

impl SolidColour {
    #[must_use]
    pub fn new(colour: Colour) -> Arc<dyn Texture> {
        Arc::new(Self { colour })
    }

    #[must_use]
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Arc<dyn Texture> {
        Self::new(Colour::new(red, green, blue))
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Vector) -> Colour {
        self.colour
    }
}

impl From<Colour> for Arc<dyn Texture> {
    fn from(colour: Colour) -> Self {
        SolidColour::new(colour)
    }
}
