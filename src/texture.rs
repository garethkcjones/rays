mod chequer;
mod image;
mod noise;
mod solidcolour;
pub use self::image::Image;
use crate::{Colour, Vec3};
pub use chequer::Chequer;
pub use noise::Noise;
pub use solidcolour::SolidColour;
use std::fmt::Debug;

/**
 * Trait for textures.
 */
pub trait Texture: Debug + Send + Sync {
    #[must_use]
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour;
}
