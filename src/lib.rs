use rand::prelude::*;

mod camera;
mod colour;
mod hit;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod vec3;
pub use camera::Camera;
pub use colour::Colour;
pub use hit::{HitRecord, Hittable, HittableList};
pub use lambertian::{Lambertian1, Lambertian2, Simple};
pub use material::Material;
pub use metal::Metal;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

/**
 * Returns a random `f64` in [0.0, 1.0).
 */
#[must_use]
pub fn random_f64() -> f64 {
    rand::thread_rng().gen()
}

/**
 * Returns a random `f64` in [min, max).
 */
#[must_use]
pub fn random_f64_in(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
