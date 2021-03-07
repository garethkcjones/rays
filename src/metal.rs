use crate::{Colour, HitRecord, Material, Ray, Vec3};

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let reflected = ray.direction.unit().reflect(normal);
        let direction = reflected + self.fuzz * Vec3::random_in_unit_sphere();
        let scattered = Ray::new(origin, direction);
        if direction.dot(normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
