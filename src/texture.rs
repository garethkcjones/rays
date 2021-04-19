mod solidcolour;
use crate::{Colour, Vec3};
pub use solidcolour::SolidColour;
use std::fmt::Debug;

/**
 * Trait for textures.
 */
pub trait Texture: Debug + Send + Sync {
    #[must_use]
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour;
}
