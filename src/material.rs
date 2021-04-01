mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;
use crate::{Colour, HitRecord, Ray, Vector};
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::{Lambertian0, Lambertian1, Lambertian2};
pub use metal::Metal;
use std::fmt::Debug;

pub trait Material: Debug + Send + Sync {
    #[must_use]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>;

    #[must_use]
    fn emitted(&self, _u: f64, _v: f64, _p: Vector) -> Colour {
        Colour::new(0.0, 0.0, 0.0)
    }
}
