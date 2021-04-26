#pragma once

#include "ray.hh"
#include "vec3.hh"

namespace rays::hittable {
	class Aabb;
}

/*
 * Type for representing an axis-aligned bounding box.
 */
class rays::hittable::Aabb final {
	public:

		constexpr explicit Aabb(Vec3 minimum, Vec3 maximum) noexcept;

		constexpr auto minimum() const noexcept {return minimum_;}
		constexpr auto maximum() const noexcept {return maximum_;}

		bool hit(Ray const &r, double t_min, double t_max) const noexcept;

	private:

		Vec3 minimum_;
		Vec3 maximum_;
};

inline constexpr rays::hittable::Aabb::Aabb(Vec3 const minimum,
                                            Vec3 const maximum) noexcept:
	minimum_{minimum},
	maximum_{maximum}
{
}
