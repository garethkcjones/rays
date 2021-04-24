#pragma once

#include <memory>
#include <optional>
#include <random>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"
#include "vec3.hh"

namespace rays::hittable {
	class Translate;
}

/*
 * Wrapper for translating hittable objects.
 */
class rays::hittable::Translate final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable>
			new_hittable(std::shared_ptr<Hittable> object, Vec3 offset);

		explicit Translate(std::shared_ptr<Hittable> object, Vec3 offset)
			noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<Hittable> object_;
		Vec3 offset_;
};
