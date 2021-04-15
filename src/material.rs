mod dielectric;
mod lambertian;
mod metal;
use crate::{Colour, HitRecord, Ray};
pub use dielectric::Dielectric;
pub use lambertian::{Lambertian0, Lambertian1, Lambertian2};
pub use metal::Metal;
use std::fmt::Debug;

/**
 * Trait for materials.
 */
pub trait Material: Debug + Send + Sync {
    #[must_use]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)>;
}
