#pragma once

#include <random>

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
			double aspect_ratio, double aperture, double focus_dist) noexcept;

		Ray get_ray(double u, double v, std::default_random_engine &rand_eng)
			const;

	private:

		Vec3 origin_;
		Vec3 horizontal_;
		Vec3 vertical_;
		Vec3 lower_left_corner_;
		Vec3 u_, v_, w_;
		double lens_radius_;
};
