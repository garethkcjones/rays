#include "camera.hh"

#include <cmath>

#include "lib.hh"
#include "vec3.hh"

using namespace rays;

Camera::Camera(Vec3 const lookfrom,
               Vec3 const lookat,
               Vec3 const vup,
               double const vfov, // Vertical field-of-view in degrees.
               double const aspect_ratio) noexcept
{
	auto const theta = degrees_to_radians(vfov);
	auto const h = std::tan(0.5 * theta);
	auto const viewport_height = 2.0 * h;
	auto const viewport_width = aspect_ratio * viewport_height;

	auto const w = (lookfrom - lookat).unit();
	auto const u = cross(vup, w).unit();
	auto const v = cross(w, u);

	origin_ = lookfrom;
	horizontal_ = viewport_width * u;
	vertical_ = viewport_height * v;
	lower_left_corner_ = origin_ - 0.5 * (horizontal_ + vertical_) - w;
}

auto Camera::get_ray(double const s, double const t) const noexcept -> Ray
{
	return Ray {
		origin_,
		lower_left_corner_ + s * horizontal_ + t * vertical_ - origin_
	};
}
