#![allow(clippy::new_ret_no_self)]

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

#[derive(Debug)]
pub struct XzRect {
    material: Arc<dyn Material>,
    x: (f64, f64),
    z: (f64, f64),
    k: f64,
}

#[derive(Debug)]
pub struct YzRect {
    material: Arc<dyn Material>,
    y: (f64, f64),
    z: (f64, f64),
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

impl XzRect {
    #[must_use]
    pub fn new(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self {
            material,
            x: (x0, x1),
            z: (z0, z1),
            k,
        })
    }
}

impl YzRect {
    #[must_use]
    pub fn new(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self {
            material,
            y: (y0, y1),
            z: (z0, z1),
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

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.y()) / r.direction.y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();
        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }

        let p = r.at(t);
        let normal = Vector::new(0.0, 1.0, 0.0);
        let material = Arc::clone(&self.material);
        let u = (x - self.x.0) / (self.x.1 - self.x.0);
        let v = (z - self.z.0) / (self.z.1 - self.z.0);

        Some(HitRecord::new(r, p, normal, material, t, u, v))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the Y dimension a
        // small amount.
        Some(Aabb::new(
            Vector::new(self.x.0, self.k - 0.0001, self.z.0),
            Vector::new(self.x.1, self.k + 0.0001, self.z.1),
        ))
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.x()) / r.direction.x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y() + t * r.direction.y();
        let z = r.origin.z() + t * r.direction.z();
        if y < self.y.0 || y > self.y.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }

        let p = r.at(t);
        let normal = Vector::new(1.0, 0.0, 0.0);
        let material = Arc::clone(&self.material);
        let u = (y - self.y.0) / (self.y.1 - self.y.0);
        let v = (z - self.z.0) / (self.z.1 - self.z.0);

        Some(HitRecord::new(r, p, normal, material, t, u, v))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        // The bounding box must have non-zero width in each dimension, so pad the X dimension a
        // small amount.
        Some(Aabb::new(
            Vector::new(self.k - 0.0001, self.y.0, self.z.0),
            Vector::new(self.k + 0.0001, self.y.1, self.z.1),
        ))
    }
}
