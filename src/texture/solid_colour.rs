use super::Texture;
use crate::{Colour, Vector};

#[derive(Clone, Debug)]
pub struct SolidColour {
    colour: Colour,
}

impl SolidColour {
    #[must_use]
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            colour: Colour::new(red, green, blue),
        }
    }
}

impl From<Colour> for SolidColour {
    fn from(colour: Colour) -> Self {
        Self { colour }
    }
}

impl From<[f64; 3]> for SolidColour {
    fn from(colour: [f64; 3]) -> Self {
        Self {
            colour: Colour::from(colour),
        }
    }
}

impl From<(f64, f64, f64)> for SolidColour {
    fn from(colour: (f64, f64, f64)) -> Self {
        Self {
            colour: Colour::from(colour),
        }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Vector) -> Colour {
        self.colour
    }
}
