mod dielectric;
mod diffuselight;
mod isotropic;
mod lambertian;
mod metal;
use crate::{Colour, HitRecord, Ray, Vec3};
pub use dielectric::Dielectric;
pub use diffuselight::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::{Lambertian0, Lambertian1, Lambertian2};
pub use metal::Metal;
use std::fmt::Debug;

/**
 * Trait for materials.
 */
pub trait Material: Debug + Send + Sync {
    #[must_use]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)>;

    #[must_use]
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Colour {
        Colour(0.0, 0.0, 0.0)
    }
}
