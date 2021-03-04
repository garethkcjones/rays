use crate::{Colour, HitRecord, Material, Ray};

#[derive(Clone, Debug)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    #[must_use]
    pub const fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        const ATTENUATION: Colour = Colour {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };

        let defraction_ratio = if rec.front_face() {
            self.ir.recip()
        } else {
            self.ir
        };

        let origin = rec.p();
        let normal = rec.normal();
        let direction = ray.direction.unit().refract(normal, defraction_ratio);
        let scattered = Ray { origin, direction };
        Some((scattered, ATTENUATION))
    }
}
