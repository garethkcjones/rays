#pragma once

#include <cassert>
#include <memory>

#include "ray.hh"
#include "vec3.hh"

namespace rays::material {
	class Material;
}

namespace rays::hittable {
	class HitRecord;
}

/*
 * Type for recording a ray hit.
 */
class rays::hittable::HitRecord final {
	public:

		explicit HitRecord(Ray const &r, Vec3 p, Vec3 normal, double t,
			double u, double v, std::shared_ptr<material::Material> material)
			noexcept;

		constexpr auto p() const noexcept {return p_;}
		constexpr auto normal() const noexcept {return normal_;}
		constexpr auto t() const noexcept {return t_;}
		constexpr auto u() const noexcept {return u_;}
		constexpr auto v() const noexcept {return v_;}
		constexpr auto front_face() const noexcept {return front_face_;}

		auto material() const noexcept {return material_;}
		material::Material &material_ref() const noexcept;

	private:

		Vec3 p_;
		Vec3 normal_;
		double t_;
		double u_, v_;
		std::shared_ptr<material::Material> material_;
		bool front_face_;
};

inline auto rays::hittable::HitRecord::material_ref() const noexcept
	-> material::Material &
{
	assert(material_);
	return *material_;
}
