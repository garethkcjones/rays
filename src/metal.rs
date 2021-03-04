use crate::{Colour, HitRecord, Material, Ray};

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Colour,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let direction = ray.direction.unit().reflect(normal);
        let scattered = Ray { origin, direction };
        if direction.dot(normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
