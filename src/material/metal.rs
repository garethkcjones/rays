#![allow(clippy::new_ret_no_self)]

use super::Material;
use crate::{Colour, HitRecord, Ray, Texture, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: impl Into<Arc<dyn Texture>>, fuzz: f64) -> Arc<dyn Material> {
        Arc::new(Self {
            albedo: albedo.into(),
            fuzz: fuzz.clamp(0.0, 1.0),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let reflected = ray.direction.unit().reflect(normal);
        let direction = reflected + self.fuzz * Vector::random_in_unit_sphere();
        let scattered = Ray::new(origin, direction, ray.time);
        if direction.dot(normal) > 0.0 {
            let colour = self.albedo.value(rec.u(), rec.v(), origin);
            Some((scattered, colour))
        } else {
            None
        }
    }
}
