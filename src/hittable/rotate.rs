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
        let Vec3(mut ox, oy, mut oz) = r.origin();
        let Vec3(mut dx, dy, mut dz) = r.direction();

        ox = self.cos_theta * ox - self.sin_theta * oz;
        oz = self.sin_theta * ox + self.cos_theta * oz;

        dx = self.cos_theta * dx - self.sin_theta * dz;
        dz = self.sin_theta * dx + self.cos_theta * dz;

        let origin = Vec3(ox, oy, oz);
        let direction = Vec3(dx, dz, dy);

        let rotated_r = Ray::new(origin, direction, r.time());

        self.object.hit(&rotated_r, tr).map(|rec| {
            let Vec3(mut px, py, mut pz) = rec.p();
            let Vec3(mut nx, ny, mut nz) = rec.normal();

            px = self.cos_theta * px + self.sin_theta * pz;
            pz = -self.sin_theta * px + self.cos_theta * pz;

            nx = self.cos_theta * nx + self.sin_theta * nz;
            nz = -self.sin_theta * nx + self.cos_theta * nz;

            let p = Vec3(px, py, pz);
            let normal = Vec3(nx, ny, nz);

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
