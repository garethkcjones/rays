#include "vec3.hh"

#include <random>

using namespace rays;

/*
 * Creates a random vector with components in the range [min, max).
 */
auto Vec3::new_random(std::default_random_engine &rand_eng,
                      double const min,
                      double const max)
	-> Vec3
{
	auto rand_dst = std::uniform_real_distribution{min, max};

	auto const x = rand_dst(rand_eng);
	auto const y = rand_dst(rand_eng);
	auto const z = rand_dst(rand_eng);

	return Vec3{x, y, z};
}

/*
 * Creates a random vector inside a unit sphere.
 */
auto Vec3::new_random_in_unit_sphere(std::default_random_engine &rand_eng)
	-> Vec3
{
	while (true) {
		auto const p = new_random(rand_eng, -1.0, 1.0);
		if (dot(p, p) < 1.0)
			return p;
	}
}

/*
 * Creates a random unit vector.
 */
auto Vec3::new_random_unit(std::default_random_engine &rand_eng) -> Vec3 {
	return new_random_in_unit_sphere(rand_eng).unit();
}

/*
 * Creates a random vector inside a hemisphere.
 */
auto Vec3::new_random_in_hemisphere(std::default_random_engine &rand_eng,
                                    Vec3 const normal)
	-> Vec3
{
	auto const in_unit_sphere = new_random_in_unit_sphere(rand_eng);
	if (dot(in_unit_sphere, normal) > 0.0) // In same hemisphere as normal.
		return in_unit_sphere;
	else
		return -in_unit_sphere;
}
