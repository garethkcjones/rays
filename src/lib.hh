#pragma once

#include <numbers>
#include <ostream>

namespace rays {
	// Utility functions.
	constexpr double degrees_to_radians(double degrees) noexcept;

	// Main raytracer function.
	void render(std::ostream &output, bool log);
}

inline constexpr auto rays::degrees_to_radians(double const degrees) noexcept
	-> double
{
	constexpr auto deg2rad = std::numbers::pi / 180.0;
	return degrees * deg2rad;
}
