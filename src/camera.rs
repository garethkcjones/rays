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
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - 0.5 * (horizontal + vertical) - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    #[must_use]
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
