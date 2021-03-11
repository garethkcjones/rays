mod solid_colour;
use crate::{Colour, Vector};
pub use solid_colour::SolidColour;

pub trait Texture {
    #[must_use]
    fn value(&self, u: f64, v: f64, p: Vector) -> Colour;
}
