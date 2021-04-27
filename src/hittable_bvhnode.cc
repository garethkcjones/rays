#include "hittable_bvhnode.hh"

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <iterator>
#include <memory>
#include <optional>
#include <random>
#include <utility>
#include <vector>

#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"

using namespace rays::hittable;

namespace {
	auto box_x_compare(std::shared_ptr<Hittable> const &a,
	                   std::shared_ptr<Hittable> const &b)
		-> bool
	{
		assert(a);
		assert(b);

		auto const box_a = a->bounding_box(0.0, 1.0);
		auto const box_b = b->bounding_box(0.0, 1.0);

		return box_a.minimum().x < box_b.minimum().x;
	}

	auto box_y_compare(std::shared_ptr<Hittable> const &a,
	                   std::shared_ptr<Hittable> const &b)
		-> bool
	{
		assert(a);
		assert(b);

		auto const box_a = a->bounding_box(0.0, 1.0);
		auto const box_b = b->bounding_box(0.0, 1.0);

		return box_a.minimum().y < box_b.minimum().y;
	}

	auto box_z_compare(std::shared_ptr<Hittable> const &a,
	                   std::shared_ptr<Hittable> const &b)
		-> bool
	{
		assert(a);
		assert(b);

		auto const box_a = a->bounding_box(0.0, 1.0);
		auto const box_b = b->bounding_box(0.0, 1.0);

		return box_a.minimum().z < box_b.minimum().z;
	}
}

BvhNode::BvhNode(std::shared_ptr<Hittable> left,
                 std::shared_ptr<Hittable> right,
                 Aabb const bounding_box) noexcept:
	left_{std::move(left)},
	right_{std::move(right)},
	bounding_box_{bounding_box}
{
	assert(left_);
	assert(right_);
}

auto BvhNode::new_hittable(std::vector<std::shared_ptr<Hittable>> objects,
                           std::size_t const start,
                           std::size_t const end,
                           double const time0,
                           double const time1,
                           std::default_random_engine &rand_eng)
	-> std::shared_ptr<Hittable>
{
	assert(!objects.empty());
	assert(start < end);
	assert(end <= objects.size());

	auto rand_dst = std::uniform_int_distribution<int>{0, 2};
	auto const axis = rand_dst(rand_eng);

	auto const comparator = (axis == 0) ? box_x_compare
	                      : (axis == 1) ? box_y_compare
	                                    : box_z_compare;

	auto const object_span = end - start;
	assert(object_span > 0);

	std::shared_ptr<Hittable> left, right;
	switch (object_span) {
		case 1:
			left = right = objects[start];
			break;

		case 2:
			if (comparator(objects[start], objects[start + 1])) {
				left = objects[start];
				right = objects[start + 1];
			} else {
				left = objects[start + 1];
				right = objects[start];
			}
			break;

		default:
			using std::begin;

			std::sort(
				std::next(begin(objects), start),
				std::next(begin(objects), end),
				comparator
			);

			auto const mid = start + object_span / 2;
			left = new_hittable(objects, start, mid, time0, time1, rand_eng);
			right = new_hittable(objects, mid, end, time0, time1, rand_eng);
	}

	assert(left);
	assert(right);

	auto const box_left = left->bounding_box(time0, time1);
	auto const box_right = right->bounding_box(time0, time1);

	auto const bounding_box = Aabb::surrounding_box(box_left, box_right);

	return std::make_shared<BvhNode>(
		std::move(left),
		std::move(right),
		bounding_box
	);
}

auto BvhNode::new_hittable(HittableList const &objects,
                           double const time0,
                           double const time1,
                           std::default_random_engine &rand_eng)
	-> std::shared_ptr<Hittable>
{
	return new_hittable(objects, 0, objects.size(), time0, time1, rand_eng);
}

auto BvhNode::hit(Ray const &r,
                  double const t_min,
                  double const t_max,
                  std::default_random_engine &rand_eng) const
	-> std::optional<HitRecord>
{
	if (!bounding_box_.hit(r, t_min, t_max))
		return std::nullopt;

	auto const hit_left = left_->hit(r, t_min, t_max, rand_eng);
	auto const hit_right =
		right_->hit(r, t_min, hit_left ? hit_left->t() : t_max, rand_eng);

	return hit_right ? hit_right : hit_left;
}

auto BvhNode::bounding_box(double /*time0*/, double /*time1*/) const -> Aabb {
	return bounding_box_;
}
