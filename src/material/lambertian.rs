use super::Material;
use crate::{Colour, HitRecord, Ray, Vector};

#[derive(Clone, Debug)]
pub struct Lambertian0 {
    albedo: Colour,
}

#[derive(Clone, Debug)]
pub struct Lambertian1 {
    albedo: Colour,
}

#[derive(Clone, Debug)]
pub struct Lambertian2 {
    albedo: Colour,
}

impl Lambertian0 {
    #[must_use]
    pub const fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Lambertian1 {
    #[must_use]
    pub const fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Lambertian2 {
    #[must_use]
    pub const fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian0 {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let direction = Vector::random_in_hemisphere(normal);
        let scattered = Ray::new(origin, direction, ray.time);
        Some((scattered, self.albedo))
    }
}

impl Material for Lambertian1 {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let mut direction = normal + Vector::random_in_unit_sphere();

        // Catch degenerate scatter direction
        if direction.near_zero() {
            direction = normal;
        }

        let scattered = Ray::new(origin, direction, ray.time);
        Some((scattered, self.albedo))
    }
}

impl Material for Lambertian2 {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let mut direction = normal + Vector::random_unit();

        // Catch degenerate scatter direction
        if direction.near_zero() {
            direction = normal;
        }

        let scattered = Ray::new(origin, direction, ray.time);
        Some((scattered, self.albedo))
    }
}
