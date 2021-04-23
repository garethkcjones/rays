use super::{HitRecord, Hittable};
use crate::{Ray, Vec3};
use std::{ops::Range, sync::Arc};

/**
 * Wrapper for rotating hittable objects about the y-axis.
 */
#[derive(Debug)]
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateY {
    #[must_use]
    pub fn new(object: Arc<dyn Hittable>, theta: f64) -> Self {
        let (sin_theta, cos_theta) = theta.to_radians().sin_cos();
        Self {
            object,
            sin_theta,
            cos_theta,
        }
    }

    #[must_use]
    pub fn new_hittable(object: Arc<dyn Hittable>, theta: f64) -> Arc<dyn Hittable> {
        Arc::new(Self::new(object, theta))
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
}
