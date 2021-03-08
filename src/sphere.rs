use crate::{HitRecord, Hittable, Material, Ray, Vec3};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Sphere {
    centre: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

#[derive(Clone, Debug)]
pub struct MovingSphere {
    centre: (Vec3, Vec3),
    time: (f64, f64),
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    #[must_use]
    pub fn new(centre: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
}

impl MovingSphere {
    #[must_use]
    pub fn new(
        centre0: Vec3,
        centre1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            centre: (centre0, centre1),
            time: (time0, time1),
            radius,
            material,
        }
    }

    #[must_use]
    pub fn centre(&self, time: f64) -> Vec3 {
        let (c0, c1) = self.centre;
        let (t0, t1) = self.time;
        c0 + ((time - t0) / (t1 - t0)) * (c1 - c0)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let centre = self.centre;
        let radius = self.radius;
        let material = Arc::clone(&self.material);

        let oc = r.origin - centre;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - radius * radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let root = root;

        let t = root;
        let p = r.at(t);
        let normal = (p - centre) / radius;
        Some(HitRecord::new(r, p, normal, material, t))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let centre = self.centre(r.time);
        let radius = self.radius;
        let material = Arc::clone(&self.material);

        let oc = r.origin - centre;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - radius * radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let root = root;

        let t = root;
        let p = r.at(t);
        let normal = (p - centre) / radius;
        Some(HitRecord::new(r, p, normal, material, t))
    }
}
