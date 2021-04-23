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
	auto [ox, oy, oz] = r.origin();
	auto [dx, dy, dz] = r.direction();

	ox = cos_theta_ * ox - sin_theta_ * oz;
	oz = sin_theta_ * ox + cos_theta_ * oz;

	dx = cos_theta_ * dx - sin_theta_ * dz;
	dz = sin_theta_ * dx + cos_theta_ * dz;

	auto const origin    = Vec3{ox, oy, oz};
	auto const direction = Vec3{dx, dz, dy};

	auto const rotated_r = Ray{origin, direction, r.time()};

	if (auto const rec = object_->hit(rotated_r, t_min, t_max); rec) {
		auto [px, py, pz] = rec->p();
		auto [nx, ny, nz] = rec->normal();

		px =  cos_theta_ * px + sin_theta_ * pz;
		pz = -sin_theta_ * px + cos_theta_ * pz;

		nx =  cos_theta_ * nx + sin_theta_ * nz;
		nz = -sin_theta_ * nx + cos_theta_ * nz;

		auto const p      = Vec3{px, py, pz};
		auto const normal = Vec3{nx, ny, nz};

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
