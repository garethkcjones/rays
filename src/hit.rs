use crate::{Ray, Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    #[must_use]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
