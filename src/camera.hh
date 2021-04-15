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

		explicit Camera(Vec3 lookfrom, Vec3 lookat, Vec3 vup, double vfov,
			double aspect_ratio) noexcept;

		Ray get_ray(double u, double v) const noexcept;

	private:

		Vec3 origin_;
		Vec3 horizontal_;
		Vec3 vertical_;
		Vec3 lower_left_corner_;
};
