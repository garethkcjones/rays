#pragma once

#include "vec3.hh"

namespace rays::hittable {
	class HitRecord;
}

/*
 * Type for recording a ray hit.
 */
class rays::hittable::HitRecord final {
	public:

		constexpr explicit HitRecord(Vec3 p, Vec3 normal, double t) noexcept;

		constexpr auto p() const noexcept {return p_;}
		constexpr auto normal() const noexcept {return normal_;}
		constexpr auto t() const noexcept {return t_;}

	private:

		Vec3 p_;
		Vec3 normal_;
		double t_;
};

inline constexpr rays::hittable::HitRecord::HitRecord(Vec3 const p,
                                                      Vec3 const normal,
                                                      double const t) noexcept:
	p_{p},
	normal_{normal},
	t_{t}
{
}
