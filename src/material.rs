mod lambertian;
use crate::{hittable::HitRecord, Colour, Ray};
pub use lambertian::{Lambertian0, Lambertian1, Lambertian2};
use std::fmt::Debug;

/**
 * Trait for materials.
 */
pub trait Material: Debug {
    #[must_use]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)>;
}
