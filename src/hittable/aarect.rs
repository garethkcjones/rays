use super::{HitRecord, Hittable};
use crate::{Material, Ray, Vec3};
use std::{ops::Range, sync::Arc};

/**
 * Type for an axis-aligned rectangle in the xy-plane.
 */
#[derive(Debug)]
pub struct XyRect {
    material: Arc<dyn Material>,
    xr: Range<f64>,
    yr: Range<f64>,
    k: f64,
}

impl XyRect {
    #[must_use]
    pub fn new(xr: Range<f64>, yr: Range<f64>, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            xr,
            yr,
            k,
        }
    }

    #[must_use]
    pub fn new_hittable(
        xr: Range<f64>,
        yr: Range<f64>,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::new(xr, yr, k, material))
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        #![allow(clippy::many_single_char_names)]

        let t = (self.k - r.origin().z()) / r.direction().z();
        if !tr.contains(&t) {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if !(self.xr.contains(&x) && self.yr.contains(&y)) {
            return None;
        }

        let u = (x - self.xr.start) / (self.xr.end - self.xr.start);
        let v = (y - self.yr.start) / (self.yr.end - self.yr.start);
        let outward_normal = Vec3(0.0, 0.0, 1.0);
        let p = r.at(t);
        let material = Arc::clone(&self.material);

        Some(HitRecord::new(r, p, outward_normal, t, u, v, material))
    }
}
