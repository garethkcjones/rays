use super::Material;
use crate::{Colour, HitRecord, Ray, Texture, Vec3};
use std::sync::Arc;

/**
 * Type for representing a pre-Lambertian scattering material.
 */
#[derive(Debug)]
pub struct Lambertian0 {
    albedo: Arc<dyn Texture>,
}

/**
 * Type for representing a pseudo-Lambertian scattering material.
 */
#[derive(Debug)]
pub struct Lambertian1 {
    albedo: Arc<dyn Texture>,
}

/**
 * Type for representing a true Lambertian scattering material.
 */
#[derive(Debug)]
pub struct Lambertian2 {
    albedo: Arc<dyn Texture>,
}

impl Lambertian0 {
    #[must_use]
    pub fn new(albedo: impl Into<Arc<dyn Texture>>) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }

    #[must_use]
    pub fn new_material(albedo: impl Into<Arc<dyn Texture>>) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Lambertian1 {
    #[must_use]
    pub fn new(albedo: impl Into<Arc<dyn Texture>>) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }

    #[must_use]
    pub fn new_material(albedo: impl Into<Arc<dyn Texture>>) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Lambertian2 {
    #[must_use]
    pub fn new(albedo: impl Into<Arc<dyn Texture>>) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }

    #[must_use]
    pub fn new_material(albedo: impl Into<Arc<dyn Texture>>) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Material for Lambertian0 {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = Vec3::new_random_in_hemisphere(rec.normal());

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.p());
        let scattered = Ray::new(rec.p(), scatter_direction, r_in.time());

        Some((attenuation, scattered))
    }
}

impl Material for Lambertian1 {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal() + Vec3::new_random_in_unit_sphere();

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.p());
        let scattered = Ray::new(rec.p(), scatter_direction, r_in.time());

        Some((attenuation, scattered))
    }
}

impl Material for Lambertian2 {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal() + Vec3::new_random_unit();

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.p());
        let scattered = Ray::new(rec.p(), scatter_direction, r_in.time());

        Some((attenuation, scattered))
    }
}
