use crate::Vec3;

/**
 * Type for recording a ray hit.
 */
#[derive(Clone, Debug)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    #[must_use]
    pub const fn new(p: Vec3, normal: Vec3, t: f64) -> Self {
        Self { p, normal, t }
    }

    #[must_use]
    pub const fn p(&self) -> Vec3 {
        self.p
    }

    #[must_use]
    pub const fn normal(&self) -> Vec3 {
        self.normal
    }

    #[must_use]
    pub const fn t(&self) -> f64 {
        self.t
    }
}
