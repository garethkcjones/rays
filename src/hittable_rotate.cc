#include "hittable_rotate.hh"

#include <cassert>
#include <cmath>
#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "lib.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;

RotateX::RotateX(std::shared_ptr<Hittable> object, double const theta) noexcept:
	object_{std::move(object)},
	sin_theta_{std::sin(theta)},
	cos_theta_{std::cos(theta)}
{
	assert(object_);
}

RotateY::RotateY(std::shared_ptr<Hittable> object, double const theta) noexcept:
	object_{std::move(object)},
	sin_theta_{std::sin(theta)},
	cos_theta_{std::cos(theta)}
{
	assert(object_);
}

RotateZ::RotateZ(std::shared_ptr<Hittable> object, double const theta) noexcept:
	object_{std::move(object)},
	sin_theta_{std::sin(theta)},
	cos_theta_{std::cos(theta)}
{
	assert(object_);
}

auto RotateX::new_hittable(std::shared_ptr<Hittable> object, double const angle)
	-> std::shared_ptr<Hittable>
{
	auto const theta = degrees_to_radians(angle);
	return std::make_shared<RotateX>(std::move(object), theta);
}

auto RotateY::new_hittable(std::shared_ptr<Hittable> object, double const angle)
	-> std::shared_ptr<Hittable>
{
	auto const theta = degrees_to_radians(angle);
	return std::make_shared<RotateY>(std::move(object), theta);
}

auto RotateZ::new_hittable(std::shared_ptr<Hittable> object, double const angle)
	-> std::shared_ptr<Hittable>
{
	auto const theta = degrees_to_radians(angle);
	return std::make_shared<RotateZ>(std::move(object), theta);
}

auto RotateX::hit(Ray const &r,
                  double const t_min,
                  double const t_max,
                  std::default_random_engine &rand_eng) const
	-> std::optional<HitRecord>
{
	auto const [ox, o1y, o1z] = r.origin();
	auto const [dx, d1y, d1z] = r.direction();

	auto const o2y = cos_theta_ * o1y - sin_theta_ * o1z;
	auto const o2z = sin_theta_ * o1y + cos_theta_ * o1z;

	auto const d2y = cos_theta_ * d1y - sin_theta_ * d1z;
	auto const d2z = sin_theta_ * d1y + cos_theta_ * d1z;

	auto const origin    = Vec3{ox, o2y, o2z};
	auto const direction = Vec3{dx, d2y, d2z};

	auto const rotated_r = Ray{origin, direction, r.time()};

	if (auto const rec = object_->hit(rotated_r, t_min, t_max, rand_eng); rec) {
		auto const [px, p1y, p1z] = rec->p();
		auto const [nx, n1y, n1z] = rec->normal();

		auto const p2y =  cos_theta_ * p1y + sin_theta_ * p1z;
		auto const p2z = -sin_theta_ * p1y + cos_theta_ * p1z;

		auto const n2y =  cos_theta_ * n1y + sin_theta_ * n1z;
		auto const n2z = -sin_theta_ * n1y + cos_theta_ * n1z;

		auto const p      = Vec3{px, p2y, p2z};
		auto const normal = Vec3{nx, n2y, n2z};

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

auto RotateY::hit(Ray const &r,
                  double const t_min,
                  double const t_max,
                  std::default_random_engine &rand_eng) const
	-> std::optional<HitRecord>
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

	if (auto const rec = object_->hit(rotated_r, t_min, t_max, rand_eng); rec) {
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

auto RotateZ::hit(Ray const &r,
                  double const t_min,
                  double const t_max,
                  std::default_random_engine &rand_eng) const
	-> std::optional<HitRecord>
{
	auto const [o1x, o1y, oz] = r.origin();
	auto const [d1x, d1y, dz] = r.direction();

	auto const o2x = cos_theta_ * o1x - sin_theta_ * o1y;
	auto const o2y = sin_theta_ * o1x + cos_theta_ * o1y;

	auto const d2x = cos_theta_ * d1x - sin_theta_ * d1y;
	auto const d2y = sin_theta_ * d1x + cos_theta_ * d1y;

	auto const origin    = Vec3{o2x, o2y, oz};
	auto const direction = Vec3{d2x, d2y, dz};

	auto const rotated_r = Ray{origin, direction, r.time()};

	if (auto const rec = object_->hit(rotated_r, t_min, t_max, rand_eng); rec) {
		auto const [p1x, p1y, pz] = rec->p();
		auto const [n1x, n1y, nz] = rec->normal();

		auto const p2x =  cos_theta_ * p1x + sin_theta_ * p1y;
		auto const p2y = -sin_theta_ * p1x + cos_theta_ * p1y;

		auto const n2x =  cos_theta_ * n1x + sin_theta_ * n1y;
		auto const n2y = -sin_theta_ * n1x + cos_theta_ * n1y;

		auto const p      = Vec3{p2x, p2y, pz};
		auto const normal = Vec3{n2x, n2y, nz};

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
