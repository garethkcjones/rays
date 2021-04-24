use super::Material;
use crate::{Colour, HitRecord, Ray, Texture, Vec3};
use std::sync::Arc;

/**
 * Type for materials that scatter randomly.
 */
#[derive(Debug)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
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

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let scattered = Ray::new(rec.p(), Vec3::new_random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.p());
        Some((attenuation, scattered))
    }
}
