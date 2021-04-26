mod aabb;
mod aarect;
mod block;
mod constantmedium;
mod hitrecord;
mod rotate;
mod sphere;
mod translate;
use crate::Ray;
use aabb::Aabb;
pub use aarect::{XyRect, XzRect, YzRect};
pub use block::Block;
pub use constantmedium::ConstantMedium;
pub use hitrecord::HitRecord;
pub use rotate::{RotateX, RotateY, RotateZ};
pub use sphere::{MovingSphere, Sphere};
use std::{fmt::Debug, ops::Range, sync::Arc};
pub use translate::Translate;

/**
 * Trait for hittable objects.
 */
pub trait Hittable: Debug + Send + Sync {
    #[must_use]
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord>;

    #[must_use]
    fn bounding_box(&self, _tr: Range<f64>) -> Aabb {
        unimplemented!() // FIXME: Remove “implementation”.
    }
}

impl Hittable for [Arc<dyn Hittable>] {
    fn hit(&self, r: &Ray, t: Range<f64>) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t.end;

        for object in self {
            if let Some(temp_rec) = object.hit(r, t.start..closest_so_far) {
                closest_so_far = temp_rec.t();
                rec = Some(temp_rec);
            }
        }

        rec
    }
}

impl Hittable for Vec<Arc<dyn Hittable>> {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        self.as_slice().hit(r, tr)
    }
}
