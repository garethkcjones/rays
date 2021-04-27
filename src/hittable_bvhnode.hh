#pragma once

#include <cstddef>
#include <memory>
#include <optional>
#include <random>
#include <vector>

#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"

namespace rays::hittable {
	class BvhNode;
}

/*
 * Type for a node in a bounding volumn hierarchy tree.
 */
class rays::hittable::BvhNode final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable>
			new_hittable(std::vector<std::shared_ptr<Hittable>> objects,
			std::size_t start, std::size_t end, double time0, double time1,
			std::default_random_engine &rand_eng);
		static std::shared_ptr<Hittable>
			new_hittable(HittableList const &objects, double time0,
			double time1, std::default_random_engine &rand_eng);

		explicit BvhNode(std::shared_ptr<Hittable> left,
			std::shared_ptr<Hittable> right, Aabb bounding_box) noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max,
			std::default_random_engine &rand_eng) const override;

		Aabb bounding_box(double time0, double time1) const override;

	private:

		std::shared_ptr<Hittable> left_, right_;
		Aabb bounding_box_;
};
