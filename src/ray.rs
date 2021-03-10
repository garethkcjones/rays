use crate::Vector;

#[derive(Clone, Default, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
    pub time: f64,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Vector, direction: Vector, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Vector {
        self.origin + t * self.direction
    }
}
