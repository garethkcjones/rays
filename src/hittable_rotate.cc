#include "hittable_rotate.hh"

#include <algorithm>
#include <cassert>
#include <cmath>
#include <limits>
#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "lib.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;

namespace {
	constexpr auto infinity = std::numeric_limits<double>::infinity();
}

RotateX::RotateX(std::shared_ptr<Hittable> object, double const theta) noexcept:
	object_{std::move(object)},
	sin_theta_{std::sin(theta)},
	cos_theta_{std::cos(theta)},
	bounding_box_{object_ ? object_->bounding_box(0.0, 1.0)
	                      : Aabb{Vec3{0.0, 0.0, 0.0}, Vec3{0.0, 0.0, 0.0}}}
{
	assert(object_);

	auto const bbmn = bounding_box_.minimum();
	auto const bbmx = bounding_box_.maximum();

	auto mnx =  infinity;
	auto mny =  infinity;
	auto mnz =  infinity;
	auto mxx = -infinity;
	auto mxy = -infinity;
	auto mxz = -infinity;

	for (auto i = 0; i < 2; ++i) {
		for (auto j = 0; j < 2; ++j) {
			for (auto k = 0; k < 2; ++k) {
				auto const x = i * bbmx.x + (1 - i) * bbmn.x;
				auto const y = j * bbmx.y + (1 - j) * bbmn.y;
				auto const z = k * bbmx.z + (1 - k) * bbmn.z;

				auto const newy =  cos_theta_ * y + sin_theta_ * z;
				auto const newz = -sin_theta_ * y + cos_theta_ * z;

				mnx = std::min(mnx, x);
				mxx = std::max(mxx, x);
				mny = std::min(mny, newy);
				mxy = std::max(mxy, newy);
				mnz = std::min(mnz, newz);
				mxz = std::max(mxz, newz);
			}
		}
	}

	auto const minimum = Vec3{mnx, mny, mnz};
	auto const maximum = Vec3{mxx, mxy, mxz};
	bounding_box_ = Aabb{minimum, maximum};
}

RotateY::RotateY(std::shared_ptr<Hittable> object, double const theta) noexcept:
	object_{std::move(object)},
	sin_theta_{std::sin(theta)},
	cos_theta_{std::cos(theta)},
	bounding_box_{object_ ? object_->bounding_box(0.0, 1.0)
	                      : Aabb{Vec3{0.0, 0.0, 0.0}, Vec3{0.0, 0.0, 0.0}}}
{
	assert(object_);

	auto const bbmn = bounding_box_.minimum();
	auto const bbmx = bounding_box_.maximum();

	auto mnx =  infinity;
	auto mny =  infinity;
	auto mnz =  infinity;
	auto mxx = -infinity;
	auto mxy = -infinity;
	auto mxz = -infinity;

	for (auto i = 0; i < 2; ++i) {
		for (auto j = 0; j < 2; ++j) {
			for (auto k = 0; k < 2; ++k) {
				auto const x = i * bbmx.x + (1 - i) * bbmn.x;
				auto const y = j * bbmx.y + (1 - j) * bbmn.y;
				auto const z = k * bbmx.z + (1 - k) * bbmn.z;

				auto const newx =  cos_theta_ * x + sin_theta_ * z;
				auto const newz = -sin_theta_ * x + cos_theta_ * z;

				mnx = std::min(mnx, newx);
				mxx = std::max(mxx, newx);
				mny = std::min(mny, y);
				mxy = std::max(mxy, y);
				mnz = std::min(mnz, newz);
				mxz = std::max(mxz, newz);
			}
		}
	}

	auto const minimum = Vec3{mnx, mny, mnz};
	auto const maximum = Vec3{mxx, mxy, mxz};
	bounding_box_ = Aabb{minimum, maximum};
}

RotateZ::RotateZ(std::shared_ptr<Hittable> object, double const theta) noexcept:
	object_{std::move(object)},
	sin_theta_{std::sin(theta)},
	cos_theta_{std::cos(theta)},
	bounding_box_{object_ ? object_->bounding_box(0.0, 1.0)
	                      : Aabb{Vec3{0.0, 0.0, 0.0}, Vec3{0.0, 0.0, 0.0}}}
{
	assert(object_);

	auto const bbmn = bounding_box_.minimum();
	auto const bbmx = bounding_box_.maximum();

	auto mnx =  infinity;
	auto mny =  infinity;
	auto mnz =  infinity;
	auto mxx = -infinity;
	auto mxy = -infinity;
	auto mxz = -infinity;

	for (auto i = 0; i < 2; ++i) {
		for (auto j = 0; j < 2; ++j) {
			for (auto k = 0; k < 2; ++k) {
				auto const x = i * bbmx.x + (1 - i) * bbmn.x;
				auto const y = j * bbmx.y + (1 - j) * bbmn.y;
				auto const z = k * bbmx.z + (1 - k) * bbmn.z;

				auto const newx =  cos_theta_ * x + sin_theta_ * y;
				auto const newy = -sin_theta_ * x + cos_theta_ * y;

				mnx = std::min(mnx, newx);
				mxx = std::max(mxx, newx);
				mny = std::min(mny, newy);
				mxy = std::max(mxy, newy);
				mnz = std::min(mnz, z);
				mxz = std::max(mxz, z);
			}
		}
	}

	auto const minimum = Vec3{mnx, mny, mnz};
	auto const maximum = Vec3{mxx, mxy, mxz};
	bounding_box_ = Aabb{minimum, maximum};
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

auto RotateX::bounding_box(double /*time0*/, double /*time1*/) const -> Aabb {
	return bounding_box_;
}

auto RotateY::bounding_box(double /*time0*/, double /*time1*/) const -> Aabb {
	return bounding_box_;
}

auto RotateZ::bounding_box(double /*time0*/, double /*time1*/) const -> Aabb {
	return bounding_box_;
}
