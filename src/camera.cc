#include "camera.hh"

#include "vec3.hh"

using namespace rays;

Camera::Camera(Vec3 const origin,
               double const viewport_width,
               double const viewport_height,
               double const focal_length) noexcept:
	origin_{origin},
	horizontal_{viewport_width, 0.0, 0.0},
	vertical_{0.0, viewport_height, 0.0},
	lower_left_corner_ {
		origin - 0.5 * (horizontal_ + vertical_) - Vec3{0.0, 0.0, focal_length}
	}
{
}
