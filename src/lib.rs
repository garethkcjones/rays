use rand::prelude::*;

mod aabb;
mod bvh;
mod camera;
mod dielectric;
mod hit;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod triplet;
pub use aabb::Aabb;
pub use bvh::BvhNode;
pub use camera::Camera;
pub use dielectric::Dielectric;
pub use hit::{HitRecord, Hittable, HittableList};
pub use lambertian::{Lambertian1, Lambertian2, Simple};
pub use material::Material;
pub use metal::Metal;
pub use ray::Ray;
pub use sphere::{MovingSphere, Sphere};
pub use triplet::{Colour, Vector};

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

/**
 * Returns a random `i32` in [min, max].
 */
#[must_use]
pub fn random_i32_in(min: i32, max: i32) -> i32 {
    random_f64_in(f64::from(min), f64::from(max) + 1.0) as i32
}
