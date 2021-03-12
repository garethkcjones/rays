use rand::prelude::*;

mod bounding;
mod camera;
mod hit;
mod material;
mod perlin;
mod ray;
mod texture;
mod triplet;
pub use bounding::{Aabb, BvhNode};
pub use camera::Camera;
pub use hit::{HitRecord, Hittable, HittableList, MovingSphere, Sphere};
pub use material::{Dielectric, Lambertian0, Lambertian1, Lambertian2, Material, Metal};
pub use perlin::Perlin;
pub use ray::Ray;
pub use texture::{Chequered, SolidColour, Texture};
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
    random_f64_in(f64::from(min), f64::from(max) + 1.0) as _
}

/**
 * Returns a random `usize` in [min, max].
 */
#[must_use]
pub fn random_usize_in(min: usize, max: usize) -> usize {
    random_f64_in(min as f64, max as f64 + 1.0) as _
}
