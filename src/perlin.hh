#pragma once

#include <array>
#include <cstddef>
#include <random>

#include "vec3.hh"

namespace rays {
	class Perlin;
}

/*
 * Type for generating Perlin noise.
 */
class rays::Perlin final {
	public:

		static constexpr std::size_t point_count = 1 << 8;

		explicit Perlin(std::default_random_engine &rand_eng);

		double noise(Vec3 p) const noexcept;
		double turb(Vec3 p, int depth = 7) const noexcept;

	private:

		std::array<Vec3, point_count> ranvec_;
		std::array<std::size_t, point_count> perm_x_, perm_y_, perm_z_;
};
