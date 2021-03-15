use super::Material;
use crate::{Colour, HitRecord, Ray, SolidColour, Texture, Vector};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Lambertian0 {
    albedo: Arc<dyn Texture>,
}

#[derive(Clone, Debug)]
pub struct Lambertian1 {
    albedo: Arc<dyn Texture>,
}

#[derive(Clone, Debug)]
pub struct Lambertian2 {
    albedo: Arc<dyn Texture>,
}

impl Lambertian0 {
    #[must_use]
    pub fn new(albedo: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }

    #[must_use]
    pub fn with_colour(colour: Colour) -> Arc<Self> {
        Self::new(Arc::new(SolidColour::from(colour)))
    }
}

impl Lambertian1 {
    #[must_use]
    pub fn new(albedo: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }

    #[must_use]
    pub fn with_colour(colour: Colour) -> Arc<Self> {
        Self::new(Arc::new(SolidColour::from(colour)))
    }
}

impl Lambertian2 {
    #[must_use]
    pub fn new(albedo: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }

    #[must_use]
    pub fn with_colour(colour: Colour) -> Arc<Self> {
        Self::new(Arc::new(SolidColour::from(colour)))
    }
}

impl Material for Lambertian0 {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let direction = Vector::random_in_hemisphere(normal);

        let scattered = Ray::new(origin, direction, ray.time);
        let colour = self.albedo.value(rec.u(), rec.v(), origin);

        Some((scattered, colour))
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
        let colour = self.albedo.value(rec.u(), rec.v(), origin);

        Some((scattered, colour))
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
        let colour = self.albedo.value(rec.u(), rec.v(), origin);

        Some((scattered, colour))
    }
}
