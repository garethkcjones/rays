use super::{HitRecord, Hittable};
use crate::{Ray, Vec3};
use std::{ops::Range, sync::Arc};

/**
 * Type for representing stationary spheres.
 */
#[derive(Debug)]
pub struct Sphere {
    centre: Vec3,
    radius: f64,
}

impl Sphere {
    #[must_use]
    pub const fn new(centre: Vec3, radius: f64) -> Self {
        Self { centre, radius }
    }

    #[must_use]
    pub fn new_hittable(centre: Vec3, radius: f64) -> Arc<dyn Hittable> {
        Arc::new(Self::new(centre, radius))
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        #![allow(clippy::many_single_char_names)]

        let oc = r.origin() - self.centre;
        let a = r.direction().dot(r.direction());
        let half_b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !tr.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !tr.contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.centre) / self.radius;

        Some(HitRecord::new(r, p, outward_normal, t))
    }
}
