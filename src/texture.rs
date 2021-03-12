mod chequered;
mod noise;
mod solid_colour;
use crate::{Colour, Vector};
pub use chequered::Chequered;
pub use noise::Noise;
pub use solid_colour::SolidColour;
use std::fmt::Debug;

pub trait Texture: Debug + Send + Sync {
    #[must_use]
    fn value(&self, u: f64, v: f64, p: Vector) -> Colour;
}
