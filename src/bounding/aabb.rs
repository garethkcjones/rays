use crate::{Ray, Vector};
use std::mem;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Aabb {
    pub minimum: Vector,
    pub maximum: Vector,
}

impl Aabb {
    #[must_use]
    pub fn new(minimum: Vector, maximum: Vector) -> Self {
        Self { minimum, maximum }
    }

    #[must_use]
    pub fn surrounding_box(box0: Option<Self>, box1: Option<Self>) -> Option<Self> {
        match (box0, box1) {
            (Some(box0), Some(box1)) => {
                let min_box = box0.minimum.apply_with(box1.minimum, f64::min);
                let max_box = box0.maximum.apply_with(box1.maximum, f64::max);
                Some(Self::new(min_box, max_box))
            }
            (Some(box0), None) | (None, Some(box0)) => Some(box0),
            _ => None,
        }
    }

    #[must_use]
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = self.minimum[a] - r.origin[a] * inv_d;
            let mut t1 = self.maximum[a] - r.origin[a] * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            let t_min = t_min.max(t0);
            let t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
