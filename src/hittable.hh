#pragma once

#include <memory>
#include <optional>
#include <vector>

#include "hittable_hitrecord.hh"
#include "ray.hh"

namespace rays::hittable {
	class Hittable;
	class HittableList;
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

/*
 * Hittable object type for storing a list of hittable objects.
 */
class rays::hittable::HittableList final:
	public Hittable,
	public std::vector<std::shared_ptr<Hittable>>
{
public:

	using vector::vector;

	std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max) const
		noexcept override;
};
