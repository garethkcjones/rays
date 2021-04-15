mod hitrecord;
mod sphere;
use crate::Ray;
pub use hitrecord::HitRecord;
pub use sphere::Sphere;
use std::{fmt::Debug, sync::Arc};

/**
 * Trait for hittable objects.
 */
pub trait Hittable: Debug + Send + Sync {
    #[must_use]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for [Arc<dyn Hittable>] {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t();
                rec = Some(temp_rec);
            }
        }

        rec
    }
}

impl Hittable for Vec<Arc<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_slice().hit(r, t_min, t_max)
    }
}
