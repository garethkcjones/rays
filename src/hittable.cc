#include "hittable.hh"

#include <cassert>
#include <optional>
#include <random>

#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"

using namespace rays::hittable;

auto HittableList::hit(Ray const &r,
                       double const t_min,
                       double const t_max,
                       std::default_random_engine &rand_eng) const
	-> std::optional<HitRecord>
{
	std::optional<HitRecord> rec;
	auto closest_so_far = t_max;

	for (auto &&object : *this) {
		assert(object);
		auto const temp_rec = object->hit(r, t_min, closest_so_far, rand_eng);
		if (temp_rec) {
			closest_so_far = temp_rec->t();
			rec = temp_rec;
		}
	}

	return rec;
}

auto HittableList::bounding_box(double const time0, double const time1) const
	-> Aabb
{
	switch (size()) {
		case 0:
			return Aabb{Vec3{0.0, 0.0, 0.0}, Vec3{0.0, 0.0, 0.0}};

		case 1:
			return front()->bounding_box(time0, time1);

		default:
			using std::begin;
			using std::end;

			auto object = begin(*this);
			auto output_box = (*object)->bounding_box(time0, time1);

			for (++object; object < end(*this); ++object) {
				auto const temp_box = (*object)->bounding_box(time0, time1);
				output_box = Aabb::surrounding_box(output_box, temp_box);
			}

			return output_box;
	}
}
