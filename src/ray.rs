use crate::Vec3;

#[derive(Clone, Default, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
