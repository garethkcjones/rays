#include "camera.hh"

#include <cassert>
#include <cmath>
#include <random>

#include "lib.hh"
#include "vec3.hh"

using namespace rays;

Camera::Camera(Vec3 const lookfrom,
               Vec3 const lookat,
               Vec3 const vup,
               double const vfov,
               double const aspect_ratio,
               double const aperture,
               double const focus_dist,
               double const time0,
               double const time1):
	time_range_{time0, time1}
{
	assert(time0 < time1);

	auto const theta = degrees_to_radians(vfov);
	auto const h = std::tan(0.5 * theta);
	auto const viewport_height = 2.0 * h;
	auto const viewport_width = aspect_ratio * viewport_height;

	w_ = (lookfrom - lookat).unit();
	u_ = cross(vup, w_).unit();
	v_ = cross(w_, u_);

	origin_ = lookfrom;
	horizontal_ = focus_dist * viewport_width * u_;
	vertical_ = focus_dist * viewport_height * v_;
	lower_left_corner_ =
		origin_ - 0.5 * (horizontal_ + vertical_) - focus_dist * w_;

	lens_radius_ = 0.5 * aperture;
}

auto Camera::get_ray(double const s,
                     double const t,
                     std::default_random_engine &rand_eng) const -> Ray
{
	auto const rd = lens_radius_ * Vec3::new_random_in_unit_disk(rand_eng);
	auto const offset = u_ * rd.x + v_ * rd.y;

	return Ray {
		origin_ + offset,
		lower_left_corner_ + s * horizontal_ + t * vertical_ - origin_ - offset,
		time_range_(rand_eng)
	};
}
