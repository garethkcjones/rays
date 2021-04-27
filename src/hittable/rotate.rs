use super::{Aabb, HitRecord, Hittable};
use crate::{Ray, Vec3};
use std::{ops::Range, sync::Arc};

/**
 * Wrapper for rotating hittable objects about the x-axis.
 */
#[derive(Debug)]
pub struct RotateX {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Aabb,
}

/**
 * Wrapper for rotating hittable objects about the y-axis.
 */
#[derive(Debug)]
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Aabb,
}

/**
 * Wrapper for rotating hittable objects about the z-axis.
 */
#[derive(Debug)]
pub struct RotateZ {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Aabb,
}

impl RotateX {
    #[must_use]
    pub fn new(object: Arc<dyn Hittable>, theta: f64) -> Self {
        let (sin_theta, cos_theta) = theta.to_radians().sin_cos();

        let bounding_box = object.bounding_box(0.0..1.0);
        let bbmn = bounding_box.minimum();
        let bbmx = bounding_box.maximum();

        let mut mnx = f64::INFINITY;
        let mut mny = f64::INFINITY;
        let mut mnz = f64::INFINITY;
        let mut mxx = -f64::INFINITY;
        let mut mxy = -f64::INFINITY;
        let mut mxz = -f64::INFINITY;

        for i in 0..2 {
            let i = f64::from(i);
            for j in 0..2 {
                let j = f64::from(j);
                for k in 0..2 {
                    let k = f64::from(k);

                    let x = i * bbmx.x() + (1.0 - i) * bbmn.x();
                    let y = j * bbmx.y() + (1.0 - j) * bbmn.y();
                    let z = k * bbmx.z() + (1.0 - k) * bbmn.z();

                    let newy = cos_theta * y + sin_theta * z;
                    let newz = -sin_theta * y + cos_theta * z;

                    mnx = mnx.min(x);
                    mxx = mxx.max(x);
                    mny = mny.min(newy);
                    mxy = mxy.max(newy);
                    mnz = mnz.min(newz);
                    mxz = mxz.max(newz);
                }
            }
        }

        let minimum = Vec3(mnx, mny, mnz);
        let maximum = Vec3(mxx, mxy, mxz);
        let bounding_box = Aabb::new(minimum, maximum);

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }

    #[must_use]
    pub fn new_hittable(object: Arc<dyn Hittable>, theta: f64) -> Arc<dyn Hittable> {
        Arc::new(Self::new(object, theta))
    }
}

impl RotateY {
    #[must_use]
    pub fn new(object: Arc<dyn Hittable>, theta: f64) -> Self {
        let (sin_theta, cos_theta) = theta.to_radians().sin_cos();

        let bounding_box = object.bounding_box(0.0..1.0);
        let bbmn = bounding_box.minimum();
        let bbmx = bounding_box.maximum();

        let mut mnx = f64::INFINITY;
        let mut mny = f64::INFINITY;
        let mut mnz = f64::INFINITY;
        let mut mxx = -f64::INFINITY;
        let mut mxy = -f64::INFINITY;
        let mut mxz = -f64::INFINITY;

        for i in 0..2 {
            let i = f64::from(i);
            for j in 0..2 {
                let j = f64::from(j);
                for k in 0..2 {
                    let k = f64::from(k);

                    let x = i * bbmx.x() + (1.0 - i) * bbmn.x();
                    let y = j * bbmx.y() + (1.0 - j) * bbmn.y();
                    let z = k * bbmx.z() + (1.0 - k) * bbmn.z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    mnx = mnx.min(newx);
                    mxx = mxx.max(newx);
                    mny = mny.min(y);
                    mxy = mxy.max(y);
                    mnz = mnz.min(newz);
                    mxz = mxz.max(newz);
                }
            }
        }

        let minimum = Vec3(mnx, mny, mnz);
        let maximum = Vec3(mxx, mxy, mxz);
        let bounding_box = Aabb::new(minimum, maximum);

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }

    #[must_use]
    pub fn new_hittable(object: Arc<dyn Hittable>, theta: f64) -> Arc<dyn Hittable> {
        Arc::new(Self::new(object, theta))
    }
}

impl RotateZ {
    #[must_use]
    pub fn new(object: Arc<dyn Hittable>, theta: f64) -> Self {
        let (sin_theta, cos_theta) = theta.to_radians().sin_cos();

        let bounding_box = object.bounding_box(0.0..1.0);
        let bbmn = bounding_box.minimum();
        let bbmx = bounding_box.maximum();

        let mut mnx = f64::INFINITY;
        let mut mny = f64::INFINITY;
        let mut mnz = f64::INFINITY;
        let mut mxx = -f64::INFINITY;
        let mut mxy = -f64::INFINITY;
        let mut mxz = -f64::INFINITY;

        for i in 0..2 {
            let i = f64::from(i);
            for j in 0..2 {
                let j = f64::from(j);
                for k in 0..2 {
                    let k = f64::from(k);

                    let x = i * bbmx.x() + (1.0 - i) * bbmn.x();
                    let y = j * bbmx.y() + (1.0 - j) * bbmn.y();
                    let z = k * bbmx.z() + (1.0 - k) * bbmn.z();

                    let newx = cos_theta * x + sin_theta * y;
                    let newy = -sin_theta * x + cos_theta * y;

                    mnx = mnx.min(newx);
                    mxx = mxx.max(newx);
                    mny = mny.min(newy);
                    mxy = mxy.max(newy);
                    mnz = mnz.min(z);
                    mxz = mxz.max(z);
                }
            }
        }

        let minimum = Vec3(mnx, mny, mnz);
        let maximum = Vec3(mxx, mxy, mxz);
        let bounding_box = Aabb::new(minimum, maximum);

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }

    #[must_use]
    pub fn new_hittable(object: Arc<dyn Hittable>, theta: f64) -> Arc<dyn Hittable> {
        Arc::new(Self::new(object, theta))
    }
}

impl Hittable for RotateX {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        let Vec3(ox, o1y, o1z) = r.origin();
        let Vec3(dx, d1y, d1z) = r.direction();

        let o2y = self.cos_theta * o1y - self.sin_theta * o1z;
        let o2z = self.sin_theta * o1y + self.cos_theta * o1z;

        let d2y = self.cos_theta * d1y - self.sin_theta * d1z;
        let d2z = self.sin_theta * d1y + self.cos_theta * d1z;

        let origin = Vec3(ox, o2y, o2z);
        let direction = Vec3(dx, d2y, d2z);

        let rotated_r = Ray::new(origin, direction, r.time());

        self.object.hit(&rotated_r, tr).map(|rec| {
            let Vec3(px, p1y, p1z) = rec.p();
            let Vec3(nx, n1y, n1z) = rec.normal();

            let p2y = self.cos_theta * p1y + self.sin_theta * p1z;
            let p2z = -self.sin_theta * p1y + self.cos_theta * p1z;

            let n2y = self.cos_theta * n1y + self.sin_theta * n1z;
            let n2z = -self.sin_theta * n1y + self.cos_theta * n1z;

            let p = Vec3(px, p2y, p2z);
            let normal = Vec3(nx, n2y, n2z);

            HitRecord::new(
                &rotated_r,
                p,
                normal,
                rec.t(),
                rec.u(),
                rec.v(),
                rec.material(),
            )
        })
    }

    fn bounding_box(&self, _tr: Range<f64>) -> Aabb {
        self.bounding_box.clone()
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        let Vec3(o1x, oy, o1z) = r.origin();
        let Vec3(d1x, dy, d1z) = r.direction();

        let o2x = self.cos_theta * o1x - self.sin_theta * o1z;
        let o2z = self.sin_theta * o1x + self.cos_theta * o1z;

        let d2x = self.cos_theta * d1x - self.sin_theta * d1z;
        let d2z = self.sin_theta * d1x + self.cos_theta * d1z;

        let origin = Vec3(o2x, oy, o2z);
        let direction = Vec3(d2x, dy, d2z);

        let rotated_r = Ray::new(origin, direction, r.time());

        self.object.hit(&rotated_r, tr).map(|rec| {
            let Vec3(p1x, py, p1z) = rec.p();
            let Vec3(n1x, ny, n1z) = rec.normal();

            let p2x = self.cos_theta * p1x + self.sin_theta * p1z;
            let p2z = -self.sin_theta * p1x + self.cos_theta * p1z;

            let n2x = self.cos_theta * n1x + self.sin_theta * n1z;
            let n2z = -self.sin_theta * n1x + self.cos_theta * n1z;

            let p = Vec3(p2x, py, p2z);
            let normal = Vec3(n2x, ny, n2z);

            HitRecord::new(
                &rotated_r,
                p,
                normal,
                rec.t(),
                rec.u(),
                rec.v(),
                rec.material(),
            )
        })
    }

    fn bounding_box(&self, _tr: Range<f64>) -> Aabb {
        self.bounding_box.clone()
    }
}

impl Hittable for RotateZ {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        let Vec3(o1x, o1y, oz) = r.origin();
        let Vec3(d1x, d1y, dz) = r.direction();

        let o2x = self.cos_theta * o1x - self.sin_theta * o1y;
        let o2y = self.sin_theta * o1x + self.cos_theta * o1y;

        let d2x = self.cos_theta * d1x - self.sin_theta * d1y;
        let d2y = self.sin_theta * d1x + self.cos_theta * d1y;

        let origin = Vec3(o2x, o2y, oz);
        let direction = Vec3(d2x, d2y, dz);

        let rotated_r = Ray::new(origin, direction, r.time());

        self.object.hit(&rotated_r, tr).map(|rec| {
            let Vec3(p1x, p1y, pz) = rec.p();
            let Vec3(n1x, n1y, nz) = rec.normal();

            let p2x = self.cos_theta * p1x + self.sin_theta * p1y;
            let p2y = -self.sin_theta * p1x + self.cos_theta * p1y;

            let n2x = self.cos_theta * n1x + self.sin_theta * n1y;
            let n2y = -self.sin_theta * n1x + self.cos_theta * n1y;

            let p = Vec3(p2x, p2y, pz);
            let normal = Vec3(n2x, n2y, nz);

            HitRecord::new(
                &rotated_r,
                p,
                normal,
                rec.t(),
                rec.u(),
                rec.v(),
                rec.material(),
            )
        })
    }

    fn bounding_box(&self, _tr: Range<f64>) -> Aabb {
        self.bounding_box.clone()
    }
}
