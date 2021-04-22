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

/**
 * Type for an axis-aligned rectangle in the xz-plane.
 */
#[derive(Debug)]
pub struct XzRect {
    material: Arc<dyn Material>,
    xr: Range<f64>,
    zr: Range<f64>,
    k: f64,
}

/**
 * Type for an axis-aligned rectangle in the yz-plane.
 */
#[derive(Debug)]
pub struct YzRect {
    material: Arc<dyn Material>,
    yr: Range<f64>,
    zr: Range<f64>,
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

impl XzRect {
    #[must_use]
    pub fn new(xr: Range<f64>, zr: Range<f64>, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            xr,
            zr,
            k,
        }
    }

    #[must_use]
    pub fn new_hittable(
        xr: Range<f64>,
        zr: Range<f64>,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::new(xr, zr, k, material))
    }
}

impl YzRect {
    #[must_use]
    pub fn new(yr: Range<f64>, zr: Range<f64>, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            yr,
            zr,
            k,
        }
    }

    #[must_use]
    pub fn new_hittable(
        yr: Range<f64>,
        zr: Range<f64>,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::new(yr, zr, k, material))
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

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        #![allow(clippy::many_single_char_names)]

        let t = (self.k - r.origin().y()) / r.direction().y();
        if !tr.contains(&t) {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if !(self.xr.contains(&x) && self.zr.contains(&z)) {
            return None;
        }

        let u = (x - self.xr.start) / (self.xr.end - self.xr.start);
        let v = (z - self.zr.start) / (self.zr.end - self.zr.start);
        let outward_normal = Vec3(0.0, 1.0, 0.0);
        let p = r.at(t);
        let material = Arc::clone(&self.material);

        Some(HitRecord::new(r, p, outward_normal, t, u, v, material))
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        #![allow(clippy::many_single_char_names)]

        let t = (self.k - r.origin().x()) / r.direction().x();
        if !tr.contains(&t) {
            return None;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if !(self.yr.contains(&y) && self.zr.contains(&z)) {
            return None;
        }

        let u = (y - self.yr.start) / (self.yr.end - self.yr.start);
        let v = (z - self.zr.start) / (self.zr.end - self.zr.start);
        let outward_normal = Vec3(1.0, 0.0, 0.0);
        let p = r.at(t);
        let material = Arc::clone(&self.material);

        Some(HitRecord::new(r, p, outward_normal, t, u, v, material))
    }
}
