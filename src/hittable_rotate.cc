#include "hittable_rotate.hh"

#include <cassert>
#include <cmath>
#include <memory>
#include <optional>
#include <utility>

#include "hittable_hitrecord.hh"
#include "lib.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;

RotateY::RotateY(std::shared_ptr<Hittable> object, double const theta) noexcept:
	object_{std::move(object)},
	sin_theta_{std::sin(theta)},
	cos_theta_{std::cos(theta)}
{
	assert(object_);
}

auto RotateY::new_hittable(std::shared_ptr<Hittable> object, double const angle)
	-> std::shared_ptr<Hittable>
{
	auto const theta = degrees_to_radians(angle);
	return std::make_shared<RotateY>(std::move(object), theta);
}

auto RotateY::hit(Ray const &r, double const t_min, double const t_max) const
	noexcept -> std::optional<HitRecord>
{
	auto const [o1x, oy, o1z] = r.origin();
	auto const [d1x, dy, d1z] = r.direction();

	auto const o2x = cos_theta_ * o1x - sin_theta_ * o1z;
	auto const o2z = sin_theta_ * o1x + cos_theta_ * o1z;

	auto const d2x = cos_theta_ * d1x - sin_theta_ * d1z;
	auto const d2z = sin_theta_ * d1x + cos_theta_ * d1z;

	auto const origin    = Vec3{o2x, oy, o2z};
	auto const direction = Vec3{d2x, dy, d2z};

	auto const rotated_r = Ray{origin, direction, r.time()};

	if (auto const rec = object_->hit(rotated_r, t_min, t_max); rec) {
		auto const [p1x, py, p1z] = rec->p();
		auto const [n1x, ny, n1z] = rec->normal();

		auto const p2x =  cos_theta_ * p1x + sin_theta_ * p1z;
		auto const p2z = -sin_theta_ * p1x + cos_theta_ * p1z;

		auto const n2x =  cos_theta_ * n1x + sin_theta_ * n1z;
		auto const n2z = -sin_theta_ * n1x + cos_theta_ * n1z;

		auto const p      = Vec3{p2x, py, p2z};
		auto const normal = Vec3{n2x, ny, n2z};

		return std::make_optional<HitRecord>(
			rotated_r,
			p,
			normal,
			rec->t(),
			rec->u(),
			rec->v(),
			rec->material()
		);
	} else {
		return std::nullopt;
	}
}
