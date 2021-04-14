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

		explicit Camera(Vec3 origin, double viewport_width,
			double viewport_height, double focal_length) noexcept;

		constexpr Ray get_ray(double u, double v) const noexcept;

	private:

		Vec3 origin_;
		Vec3 horizontal_;
		Vec3 vertical_;
		Vec3 lower_left_corner_;
};

inline constexpr auto rays::Camera::get_ray(double const u, double const v)
	const noexcept -> Ray
{
	return Ray {
		origin_,
		lower_left_corner_ + u * horizontal_ + v * vertical_ - origin_
	};
}
