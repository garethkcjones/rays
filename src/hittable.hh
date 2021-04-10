#pragma once

#include <optional>

#include "hittable_hitrecord.hh"
#include "ray.hh"

namespace rays::hittable {
	class Hittable;
}

/*
 * Abstract type for hittable objects.
 */
class rays::hittable::Hittable {
	public:

		virtual ~Hittable() noexcept = default;

		virtual std::optional<HitRecord> hit(Ray const &r, double t_min,
		                                     double t_max) const noexcept = 0;
};
