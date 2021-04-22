use super::{HitRecord, Hittable};
use crate::{Material, Ray, Vec3};
use std::{ops::Range, sync::Arc};

/**
 * Type for representing stationary spheres.
 */
#[derive(Debug)]
pub struct Sphere {
    centre: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

/**
 * Type for representing moving spheres.
 */
#[derive(Debug)]
pub struct MovingSphere {
    centre: (Vec3, Vec3),
    time: Range<f64>,
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

    #[must_use]
    pub fn new_hittable(
        centre: Vec3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::new(centre, radius, material))
    }
}

impl MovingSphere {
    #[must_use]
    pub fn new(
        centre0: Vec3,
        centre1: Vec3,
        time: Range<f64>,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            centre: (centre0, centre1),
            time,
            radius,
            material,
        }
    }

    #[must_use]
    pub fn new_hittable(
        centre0: Vec3,
        centre1: Vec3,
        time: Range<f64>,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::new(centre0, centre1, time, radius, material))
    }

    #[must_use]
    fn centre(&self, time: f64) -> Vec3 {
        let (centre0, centre1) = self.centre;
        let time0 = self.time.start;
        let time1 = self.time.end;
        centre0 + ((time - time0) / (time1 - time0)) * (centre1 - centre0)
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
        let material = Arc::clone(&self.material);

        Some(HitRecord::new(r, p, outward_normal, t, material))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        #![allow(clippy::many_single_char_names)]

        let centre = self.centre(r.time());
        let oc = r.origin() - centre;
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
        let outward_normal = (p - centre) / self.radius;
        let material = Arc::clone(&self.material);

        Some(HitRecord::new(r, p, outward_normal, t, material))
    }
}
