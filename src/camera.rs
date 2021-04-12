use crate::{Ray, Vec3};

/**
 * Type for representing a viewport.
 */
#[derive(Clone, Debug)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    #[must_use]
    pub fn new(origin: Vec3, viewport_width: f64, viewport_height: f64, focal_length: f64) -> Self {
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - 0.5 * (horizontal + vertical) - Vec3(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    #[must_use]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
