use crate::{Colour, HitRecord, Material, Ray};

#[derive(Clone, Debug)]
pub struct Metal {
    pub albedo: Colour,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p;
        let direction = ray.direction.unit().reflect(rec.normal);
        let scattered = Ray { origin, direction };
        if direction.dot(rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
