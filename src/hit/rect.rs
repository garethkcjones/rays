use super::{HitRecord, Hittable};
use crate::{Aabb, Material, Ray, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct XyRect {
    material: Arc<dyn Material>,
    x: (f64, f64),
    y: (f64, f64),
    k: f64,
}

impl XyRect {
    #[must_use]
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self {
            material,
            x: (x0, x1),
            y: (y0, y1),
            k,
        })
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z()) / r.direction.z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x() + t * r.direction.x();
        let y = r.origin.y() + t * r.direction.y();
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }

        let p = r.at(t);
        let normal = Vector::new(0.0, 0.0, 1.0);
        let material = Arc::clone(&self.material);
        let u = (x - self.x.0) / (self.x.1 - self.x.0);
        let v = (y - self.y.0) / (self.y.1 - self.y.0);

        Some(HitRecord::new(r, p, normal, material, t, u, v))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a
        // small amount.
        Some(Aabb::new(
            Vector::new(self.x.0, self.y.0, self.k - 0.0001),
            Vector::new(self.x.1, self.y.1, self.k + 0.0001),
        ))
    }
}
