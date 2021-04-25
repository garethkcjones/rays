#include "vec3.hh"

#include <algorithm>
#include <cmath>
#include <ostream>
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

/*
 * Creates a random vector inside a unit disk.
 */
auto Vec3::new_random_in_unit_disk(std::default_random_engine &rand_eng) -> Vec3
{
	auto rand_dst = std::uniform_real_distribution{-1.0, 1.0};

	while (true) {
		auto const x = rand_dst(rand_eng);
		auto const y = rand_dst(rand_eng);

		auto const p = Vec3{x, y, 0.0};

		if (dot(p, p) < 1.0)
			return p;
	}
}

/*
 * Refracts `uv` through surface with normal `n` and refractive index ratio
 * `etai_over_etat`.
 */
auto rays::refract(Vec3 const uv, Vec3 const n, double const etai_over_etat)
	noexcept -> Vec3
{
	auto const cos_theta = std::clamp(dot(-uv, n), -1.0, 1.0);
	auto const r_out_perp = etai_over_etat * (uv + cos_theta * n);
	auto const r_out_parallel =
		-std::sqrt(std::abs(1.0 - dot(r_out_perp, r_out_perp))) * n;
	return r_out_perp + r_out_parallel;
}

auto rays::operator<<(std::ostream &os, Vec3 const v) -> std::ostream & {
	os << '(' << v.x << ", " << v.y << ", " << v.z << ')';
	return os;
}
