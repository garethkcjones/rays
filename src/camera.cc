#include "camera.hh"

#include <cmath>

#include "lib.hh"
#include "vec3.hh"

using namespace rays;

Camera::Camera(double const vfov, // Vertical field-of-view in degrees.
               double const aspect_ratio,
               Vec3 const origin,
               double const focal_length) noexcept:
	origin_{origin}
{
	auto const theta = degrees_to_radians(vfov);
	auto const h = std::tan(0.5 * theta);
	auto const viewport_height = 2.0 * h;
	auto const viewport_width = aspect_ratio * viewport_height;

	horizontal_ = Vec3{viewport_width, 0.0, 0.0};
	vertical_ = Vec3{0.0, viewport_height, 0.0};
	lower_left_corner_ =
	   origin_ - 0.5 * (horizontal_ + vertical_) - Vec3{0.0, 0.0, focal_length};
}
