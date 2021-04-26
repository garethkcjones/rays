#pragma once

#include <exception> // FIXME: Temporary
#include <memory>
#include <optional>
#include <random>
#include <vector>

#include "hittable_aabb.hh"
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
			double t_max, std::default_random_engine &rand_eng) const = 0;

		virtual Aabb bounding_box(double /*time0*/, double /*time1*/) const /* FIXME: = 0; */ {std::terminate();}
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

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max,
			std::default_random_engine &rand_eng) const override;
};
