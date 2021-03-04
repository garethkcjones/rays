use crate::{HitRecord, Hittable, Material, Ray, Vec3};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Sphere {
    centre: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    #[must_use]
    pub fn new(centre: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
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
        let material = Rc::clone(&self.material);
        Some(HitRecord::new(r, p, normal, material, t))
    }
}
