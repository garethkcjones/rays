#pragma once

#include "ray.hh"
#include "vec3.hh"

namespace rays {
	class Camera;
}

/*
 * Type for representing a viewport.
 */
class rays::Camera final {
	public:

		constexpr explicit Camera(Vec3 origin, double viewport_width,
			double viewport_height, double focal_length) noexcept;

		constexpr Ray get_ray(double u, double v) noexcept;

	private:

		Vec3 origin_;
		Vec3 horizontal_;
		Vec3 vertical_;
		Vec3 lower_left_corner_;
};

inline constexpr rays::Camera::Camera(Vec3 const origin,
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

inline constexpr auto rays::Camera::get_ray(double const u, double const v)
	noexcept -> Ray
{
	return Ray {
		origin_,
		lower_left_corner_ + u * horizontal_ + v * vertical_ - origin_
	};
}
