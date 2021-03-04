use crate::{Colour, HitRecord, Material, Ray, Vec3};

#[derive(Clone, Debug)]
pub struct Simple {
    albedo: Colour,
}

#[derive(Clone, Debug)]
pub struct Lambertian1 {
    albedo: Colour,
}

#[derive(Clone, Debug)]
pub struct Lambertian2 {
    albedo: Colour,
}

impl Simple {
    #[must_use]
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Lambertian1 {
    #[must_use]
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Lambertian2 {
    #[must_use]
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Simple {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let direction = Vec3::random_in_hemisphere(normal);
        let scattered = Ray { origin, direction };
        Some((scattered, self.albedo))
    }
}

impl Material for Lambertian1 {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let mut direction = normal + Vec3::random_in_unit_sphere();

        // Catch degenerate scatter direction
        if direction.near_zero() {
            direction = normal;
        }

        let scattered = Ray { origin, direction };
        Some((scattered, self.albedo))
    }
}

impl Material for Lambertian2 {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let origin = rec.p();
        let normal = rec.normal();
        let mut direction = normal + Vec3::random_unit();

        // Catch degenerate scatter direction
        if direction.near_zero() {
            direction = normal;
        }

        let scattered = Ray { origin, direction };
        Some((scattered, self.albedo))
    }
}
