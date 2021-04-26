#pragma once

#include <memory>
#include <optional>
#include <random>

#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

namespace rays::hittable {
	class Block;
}

/*
 * Type for hittable cuboid blocks.
 */
class rays::hittable::Block final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable> new_hittable(Vec3 box_min,
			Vec3 box_max, std::shared_ptr<material::Material> material);

		explicit Block(Vec3 box_min, Vec3 box_max,
			std::shared_ptr<material::Material> material);

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max,
			std::default_random_engine &rand_eng) const override;

		Aabb bounding_box(double time0, double time1) const override;

	private:

		Vec3 box_min_, box_max_;
		HittableList sides_;
};
