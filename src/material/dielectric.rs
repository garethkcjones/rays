use super::Material;
use crate::{random_f64, Colour, HitRecord, Ray};
use std::sync::Arc;

#[derive(Debug)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    #[must_use]
    pub fn new(ir: f64) -> Arc<Self> {
        Arc::new(Self { ir })
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let attenuation: Colour = Colour::new(1.0, 1.0, 1.0);

        let origin = rec.p();
        let normal = rec.normal();

        let refraction_ratio = if rec.front_face() {
            self.ir.recip()
        } else {
            self.ir
        };

        let unit_direction = ray.direction.unit();
        let cos_theta = (-unit_direction).dot(normal).clamp(-1.0, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let random_reflect = reflectance(cos_theta, refraction_ratio) > random_f64();

        let direction = if cannot_refract || random_reflect {
            unit_direction.reflect(normal)
        } else {
            unit_direction.refract(normal, refraction_ratio)
        };

        let scattered = Ray::new(origin, direction, ray.time);
        Some((scattered, attenuation))
    }
}

#[must_use]
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick’s approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
