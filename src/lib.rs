#![allow(clippy::many_single_char_names)]

mod bounding;
mod camera;
mod hit;
mod material;
pub mod output;
mod perlin;
pub mod random;
mod ray;
mod texture;
mod triplet;
pub use bounding::{Aabb, BvhNode};
pub use camera::Camera;
pub use hit::{HitRecord, Hittable, MovingSphere, Sphere};
pub use material::{
    Dielectric, DiffuseLight, Lambertian0, Lambertian1, Lambertian2, Material, Metal,
};
pub use perlin::Perlin;
pub use ray::Ray;
pub use texture::{Chequered, Noise, OpaqueImage, SolidColour, Texture};
pub use triplet::{Colour, Vector};
