use super::Material;
use crate::{hittable::HitRecord, Colour, Ray};
use std::sync::Arc;

/**
 * Type for representing a reflective material.
 */
#[derive(Debug)]
pub struct Metal {
    albedo: Colour,
}

impl Metal {
    #[must_use]
    pub const fn new(albedo: Colour) -> Self {
        Self { albedo }
    }

    #[must_use]
    pub fn new_material(albedo: Colour) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = r_in.direction().unit().reflect(rec.normal());
        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p(), reflected);

        if scattered.direction().dot(rec.normal()) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
