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
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        todo!()
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
