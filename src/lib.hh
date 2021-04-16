#pragma once

#include <memory>
#include <numbers>
#include <ostream>

#include "camera.hh"
#include "hittable.hh"

namespace rays {
	// Utility functions.
	constexpr double degrees_to_radians(double degrees) noexcept;

	// Main raytracer function.
	void run(int num_threads, std::shared_ptr<hittable::Hittable const> world,
		int image_width, int image_height, int samples_per_pixel, int max_depth,
		std::shared_ptr<Camera const> cam, std::ostream &output, bool log);
}

inline constexpr auto rays::degrees_to_radians(double const degrees) noexcept
	-> double
{
	constexpr auto deg2rad = std::numbers::pi / 180.0;
	return degrees * deg2rad;
}
