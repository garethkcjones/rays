use crate::{random_f64_in, Ray, Vector};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    time: (f64, f64),
    lens_radius: f64,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        look_from: Vector,
        look_at: Vector,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - focus_dist * w;

        let lens_radius = 0.5 * aperture;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            time: (time0, time1),
            lens_radius,
        }
    }

    #[must_use]
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vector::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let origin = self.origin + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;
        let time = random_f64_in(self.time.0, self.time.1);
        Ray::new(origin, direction, time)
    }
}
