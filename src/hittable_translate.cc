#include "hittable_translate.hh"

#include <cassert>
#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;

Translate::Translate(std::shared_ptr<Hittable> object, Vec3 const offset)
	noexcept:
	object_{std::move(object)},
	offset_{offset}
{
	assert(object_);
}

auto Translate::new_hittable(std::shared_ptr<Hittable> object,
                             Vec3 const offset)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<Translate>(std::move(object), offset);
}

auto Translate::hit(Ray const &r,
                    double const t_min,
                    double const t_max,
                    std::default_random_engine &rand_eng) const
	-> std::optional<HitRecord>
{
	auto const moved_r = Ray{r.origin() - offset_, r.direction(), r.time()};
	if (auto const rec = object_->hit(moved_r, t_min, t_max, rand_eng); rec) {
		return std::make_optional<HitRecord>(
			moved_r,
			rec->p() + offset_,
			rec->normal(),
			rec->t(),
			rec->u(),
			rec->v(),
			rec->material()
		);
	} else {
		return std::nullopt;
	}
}

auto Translate::bounding_box(double const time0, double const time1) const
	-> Aabb
{
	auto const output_box = object_->bounding_box(time0, time1);
	return Aabb{output_box.minimum() + offset_, output_box.maximum() + offset_};
}
