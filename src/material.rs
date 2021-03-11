mod dielectric;
mod lambertian;
mod metal;
use crate::{Colour, HitRecord, Ray};
pub use dielectric::Dielectric;
pub use lambertian::{Lambertian0, Lambertian1, Lambertian2};
pub use metal::Metal;
use std::fmt::Debug;

pub trait Material: Debug + Sync + Send {
    #[must_use]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>;
}
