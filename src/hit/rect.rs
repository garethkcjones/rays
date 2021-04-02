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
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        todo!()
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
