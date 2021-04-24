#include "hittable.hh"

#include <cassert>
#include <optional>
#include <random>

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
