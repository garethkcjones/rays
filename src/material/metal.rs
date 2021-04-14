use super::Material;
use crate::{Colour, HitRecord, Ray, Vec3};
use std::sync::Arc;

/**
 * Type for representing a reflective material.
 */
#[derive(Debug)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        assert!(fuzz >= 0.0);
        assert!(fuzz <= 1.0);
        Self { albedo, fuzz }
    }

    #[must_use]
    pub fn new_material(albedo: Colour, fuzz: f64) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo, fuzz))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = r_in.direction().unit().reflect(rec.normal());
        let attenuation = self.albedo;
        let scattered = Ray::new(
            rec.p(),
            reflected + self.fuzz * Vec3::new_random_in_unit_sphere(),
        );

        if scattered.direction().dot(rec.normal()) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
