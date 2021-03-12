use super::Material;
use crate::{Colour, HitRecord, Ray, SolidColour, Texture, Vector};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: Arc<dyn Texture>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }

    #[must_use]
    pub fn with_colour(colour: Colour, fuzz: f64) -> Self {
        Self::new(Arc::new(SolidColour::from(colour)), fuzz)
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
