use crate::{Ray, Vec3};
use ::rand::{distributions::Uniform, prelude::*};
use std::ops::Range;

/**
 * Type for representing a viewport.
 */
#[derive(Clone, Debug)]
pub struct Camera {
    time_range: Uniform<f64>,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_range: Range<f64>,
    ) -> Self {
        assert!(!time_range.is_empty());

        let theta = vfov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let time_range = Uniform::from(time_range);
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - 0.5 * (horizontal + vertical) - focus_dist * w;
        let lens_radius = 0.5 * aperture;

        Self {
            time_range,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    #[must_use]
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::new_random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            thread_rng().sample(self.time_range),
        )
    }
}
