mod hitrecord;
use crate::Ray;
pub use hitrecord::HitRecord;
use std::fmt::Debug;

/**
 * Trait for hittable objects.
 */
pub trait Hittable: Debug {
    #[must_use]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
