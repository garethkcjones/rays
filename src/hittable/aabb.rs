use crate::{Ray, Vec3};
use std::{ops::Range, mem};

/**
 * Type for representing an axis-aligned bounding box.
 */
#[derive(Clone, Debug)]
pub struct Aabb {
    minimum: Vec3,
    maximum: Vec3,
}

impl Aabb {
    #[must_use]
    pub const fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    #[must_use]
    pub const fn minimum(&self) -> Vec3 {
        self.minimum
    }

    #[must_use]
    pub const fn maximum(&self) -> Vec3 {
        self.maximum
    }

    #[must_use]
    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let Vec3(b0mnx, b0mny, b0mnz) = box0.minimum;
        let Vec3(b0mxx, b0mxy, b0mxz) = box0.maximum;
        let Vec3(b1mnx, b1mny, b1mnz) = box1.minimum;
        let Vec3(b1mxx, b1mxy, b1mxz) = box1.maximum;

        let small = Vec3(b0mnx.min(b1mnx), b0mny.min(b1mny), b0mnz.min(b1mnz));
        let big = Vec3(b0mxx.max(b1mxx), b0mxy.max(b1mxy), b0mxz.max(b1mxz));

        Aabb::new(small, big)
    }

    #[must_use]
    pub fn hit(&self, r: &Ray, tr: Range<f64>) -> bool {
        let mut t_min = tr.start;
        let mut t_max = tr.end;

        let Vec3(mnx, mny, mnz) = self.minimum;
        let Vec3(mxx, mxy, mxz) = self.maximum;

        let Vec3(ox, oy, oz) = r.origin();
        let Vec3(dx, dy, dz) = r.direction();

        hit1d(mnx, mxx, ox, dx, &mut t_min, &mut t_max)
            && hit1d(mny, mxy, oy, dy, &mut t_min, &mut t_max)
            && hit1d(mnz, mxz, oz, dz, &mut t_min, &mut t_max)
    }
}

#[must_use]
fn hit1d(
    minimum: f64,
    maximum: f64,
    origin: f64,
    direction: f64,
    t_min: &mut f64,
    t_max: &mut f64,
) -> bool {
    let inv_dir = direction.recip();

    let mut t0 = (minimum - origin) * inv_dir;
    let mut t1 = (maximum - origin) * inv_dir;

    if inv_dir < 0.0 {
        mem::swap(&mut t0, &mut t1);
    }

    *t_min = t0.max(*t_min);
    *t_max = t1.min(*t_max);

    t_min < t_max
}
