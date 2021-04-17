use super::Material;
use crate::{Colour, HitRecord, Ray};
use rand::prelude::*;
use std::sync::Arc;

/**
 * Type for representing a transparent material.
 */
#[derive(Debug)]
pub struct Dielectric {
    ir: f64, // Index of refraction.
}

impl Dielectric {
    #[must_use]
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    #[must_use]
    pub fn new_material(ir: f64) -> Arc<dyn Material> {
        Arc::new(Self::new(ir))
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let attenuation = Colour(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face() {
            self.ir.recip()
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction).dot(rec.normal()).clamp(-1.0, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > thread_rng().gen() {
                unit_direction.reflect(rec.normal())
            } else {
                unit_direction.refract(rec.normal(), refraction_ratio)
            };

        let scattered = Ray::new(rec.p(), direction, r_in.time());

        Some((attenuation, scattered))
    }
}

/**
 * Use Schlick’s approximation for reflectance.
 */
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
