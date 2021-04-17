use super::Material;
use crate::{Colour, HitRecord, Ray, Vec3};
use std::sync::Arc;

/**
 * Type for representing a pre-Lambertian scattering material.
 */
#[derive(Debug)]
pub struct Lambertian0 {
    albedo: Colour,
}

/*
 * Type for representing a pseudo-Lambertian scattering material.
 */
#[derive(Debug)]
pub struct Lambertian1 {
    albedo: Colour,
}

/*
 * Type for representing a true Lambertian scattering material.
 */
#[derive(Debug)]
pub struct Lambertian2 {
    albedo: Colour,
}

impl Lambertian0 {
    #[must_use]
    pub const fn new(albedo: Colour) -> Self {
        Self { albedo }
    }

    #[must_use]
    pub fn new_material(albedo: Colour) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Lambertian1 {
    #[must_use]
    pub const fn new(albedo: Colour) -> Self {
        Self { albedo }
    }

    #[must_use]
    pub fn new_material(albedo: Colour) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Lambertian2 {
    #[must_use]
    pub const fn new(albedo: Colour) -> Self {
        Self { albedo }
    }

    #[must_use]
    pub fn new_material(albedo: Colour) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Material for Lambertian0 {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = Vec3::new_random_in_hemisphere(rec.normal());

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p(), scatter_direction, 0.0);

        Some((attenuation, scattered))
    }
}

impl Material for Lambertian1 {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal() + Vec3::new_random_in_unit_sphere();

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p(), scatter_direction, 0.0);

        Some((attenuation, scattered))
    }
}

impl Material for Lambertian2 {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal() + Vec3::new_random_unit();

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p(), scatter_direction, 0.0);

        Some((attenuation, scattered))
    }
}
