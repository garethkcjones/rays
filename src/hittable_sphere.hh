#pragma once

#include <memory>
#include <optional>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"
#include "vec3.hh"

namespace rays::hittable {
	class Sphere;
}

/*
 * Type for representing stationary spheres.
 */
class rays::hittable::Sphere final:
	public Hittable
{
	public:
		static std::shared_ptr<Hittable> new_hittable(Vec3 centre,
		                                              double radius);

		explicit Sphere(Vec3 centre, double radius) noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max)
			const noexcept override;

	private:

		Vec3 centre_;
		double radius_;
};
