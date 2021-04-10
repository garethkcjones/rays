#pragma once

#include "ray.hh"
#include "vec3.hh"

namespace rays::hittable {
	class HitRecord;
}

/*
 * Type for recording a ray hit.
 */
class rays::hittable::HitRecord final {
	public:

		constexpr explicit HitRecord(Ray const &r, Vec3 p, Vec3 normal,
		                             double t) noexcept;

		constexpr auto p() const noexcept {return p_;}
		constexpr auto normal() const noexcept {return normal_;}
		constexpr auto t() const noexcept {return t_;}
		constexpr auto front_face() const noexcept {return front_face_;}

	private:

		Vec3 p_;
		Vec3 normal_;
		double t_;
		bool front_face_;
};

inline constexpr rays::hittable::HitRecord::HitRecord(Ray const &r,
                                                      Vec3 const p,
                                                      Vec3 const outward_normal,
                                                      double const t) noexcept:
	p_{p},
	normal_{outward_normal},
	t_{t},
	front_face_{dot(r.direction(), outward_normal) < 0.0}
{
	if (!front_face_)
		normal_ = -outward_normal;
}
