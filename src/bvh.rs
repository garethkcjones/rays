use crate::{Aabb, HitRecord, Hittable, HittableList, Ray};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct BvhNode {
    bounding_box: Aabb,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BvhNode {
    #[must_use]
    pub fn new(objects: &HittableList, time0: f64, time1: f64) -> Self {
        Self::from_range(objects.clone(), 0, objects.len(), time0, time1)
    }

    #[must_use]
    fn from_range(
        _objects: HittableList,
        _start: usize,
        _end: usize,
        _time0: f64,
        _time1: f64,
    ) -> Self {
        todo!()
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bounding_box.hit(r, t_min, t_max) {
            let left_hit = self.left.hit(r, t_min, t_max);
            let t_max = left_hit.as_ref().map_or(t_max, |rec| rec.t());
            let right_hit = self.right.hit(r, t_min, t_max);
            right_hit.or(left_hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
