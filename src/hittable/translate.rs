use super::{Aabb, HitRecord, Hittable};
use crate::{Ray, Vec3};
use std::{ops::Range, sync::Arc};

/**
 * Wrapper for translating hittable objects.
 */
#[derive(Debug)]
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    #[must_use]
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { object, offset }
    }

    #[must_use]
    pub fn new_hittable(object: Arc<dyn Hittable>, offset: Vec3) -> Arc<dyn Hittable> {
        Arc::new(Self::new(object, offset))
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        self.object.hit(&moved_r, tr).map(|rec| {
            HitRecord::new(
                &moved_r,
                rec.p() + self.offset,
                rec.normal(),
                rec.t(),
                rec.u(),
                rec.v(),
                rec.material(),
            )
        })
    }

    fn bounding_box(&self, tr: Range<f64>) -> Aabb {
        let output_box = self.object.bounding_box(tr);
        Aabb::new(
            output_box.minimum() + self.offset,
            output_box.maximum() + self.offset,
        )
    }
}
