use crate::Vec3;

/**
 * Type to represent a ray of light.
 */
#[derive(Clone, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[must_use]
    pub const fn origin(&self) -> Vec3 {
        self.origin
    }

    #[must_use]
    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    #[must_use]
    pub const fn time(&self) -> f64 {
        self.time
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
