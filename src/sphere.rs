use crate::{HitRecord, Hittable, Ray, Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.centre;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

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
        let normal = (p - self.centre) / self.radius;
        Some(HitRecord::new(r, p, normal, t))
    }
}
