use crate::{Colour, HitRecord, Material, Ray, Vec3};

#[derive(Clone, Debug)]
pub struct Simple {
    pub albedo: Colour,
}

#[derive(Clone, Debug)]
pub struct Lambertian1 {
    pub albedo: Colour,
}

#[derive(Clone, Debug)]
pub struct Lambertian2 {
    pub albedo: Colour,
}

impl Material for Simple {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p;
        let direction = Vec3::random_in_hemisphere(rec.normal);
        let scattered = Ray { origin, direction };
        Some((scattered, self.albedo))
    }
}

impl Material for Lambertian1 {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p;
        let mut direction = rec.normal + Vec3::random_in_unit_sphere();

        // Catch degenerate scatter direction
        if direction.near_zero() {
            direction = rec.normal;
        }

        let scattered = Ray { origin, direction };
        Some((scattered, self.albedo))
    }
}

impl Material for Lambertian2 {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p;
        let mut direction = rec.normal + Vec3::random_unit();

        // Catch degenerate scatter direction
        if direction.near_zero() {
            direction = rec.normal;
        }

        let scattered = Ray { origin, direction };
        Some((scattered, self.albedo))
    }
}
