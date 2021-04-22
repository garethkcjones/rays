#include "hittable_aarect.hh"

#include <cassert>
#include <memory>
#include <optional>
#include <utility>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;
using rays::material::Material;

XyRect::XyRect(double const x0,
               double const x1,
               double const y0,
               double const y1,
               double const k,
               std::shared_ptr<Material> material) noexcept:
	material_{std::move(material)},
	x0_{x0},
	x1_{x1},
	y0_{y0},
	y1_{y1},
	k_{k}
{
	assert(material_);
}

XzRect::XzRect(double const x0,
               double const x1,
               double const z0,
               double const z1,
               double const k,
               std::shared_ptr<Material> material) noexcept:
	material_{std::move(material)},
	x0_{x0},
	x1_{x1},
	z0_{z0},
	z1_{z1},
	k_{k}
{
	assert(material_);
}

YzRect::YzRect(double const y0,
               double const y1,
               double const z0,
               double const z1,
               double const k,
               std::shared_ptr<Material> material) noexcept:
	material_{std::move(material)},
	y0_{y0},
	y1_{y1},
	z0_{z0},
	z1_{z1},
	k_{k}
{
	assert(material_);
}

auto XyRect::new_hittable(double const x0,
                          double const x1,
                          double const y0,
                          double const y1,
                          double const k,
                          std::shared_ptr<Material> material)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<XyRect>(x0, x1, y0, y1, k, std::move(material));
}

auto XzRect::new_hittable(double const x0,
                          double const x1,
                          double const z0,
                          double const z1,
                          double const k,
                          std::shared_ptr<Material> material)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<XzRect>(x0, x1, z0, z1, k, std::move(material));
}

auto YzRect::new_hittable(double const y0,
                          double const y1,
                          double const z0,
                          double const z1,
                          double const k,
                          std::shared_ptr<Material> material)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<YzRect>(y0, y1, z0, z1, k, std::move(material));
}

auto XyRect::hit(Ray const &r, double const t_min, double const t_max) const
	noexcept -> std::optional<HitRecord>
{
	auto const t = (k_ - r.origin().z) / r.direction().z;
	if (t < t_min || t > t_max)
		return std::nullopt;

	auto const x = r.origin().x + t * r.direction().x;
	auto const y = r.origin().y + t * r.direction().y;
	if (x < x0_ || x > x1_ || y < y0_ || y > y1_)
		return std::nullopt;

	auto const u = (x - x0_) / (x1_ - x0_);
	auto const v = (y - y0_) / (y1_ - y0_);
	auto const outward_normal = Vec3{0.0, 0.0, 1.0};
	auto const p = r.at(t);

	return std::make_optional<HitRecord>(
		r,
		p,
		outward_normal,
		t,
		u,
		v,
		material_
	);
}

auto XzRect::hit(Ray const &r, double const t_min, double const t_max) const
	noexcept -> std::optional<HitRecord>
{
	auto const t = (k_ - r.origin().y) / r.direction().y;
	if (t < t_min || t > t_max)
		return std::nullopt;

	auto const x = r.origin().x + t * r.direction().x;
	auto const z = r.origin().z + t * r.direction().z;
	if (x < x0_ || x > x1_ || z < z0_ || z > z1_)
		return std::nullopt;

	auto const u = (x - x0_) / (x1_ - x0_);
	auto const v = (z - z0_) / (z1_ - z0_);
	auto const outward_normal = Vec3{0.0, 1.0, 0.0};
	auto const p = r.at(t);

	return std::make_optional<HitRecord>(
		r,
		p,
		outward_normal,
		t,
		u,
		v,
		material_
	);
}

auto YzRect::hit(Ray const &r, double const t_min, double const t_max) const
	noexcept -> std::optional<HitRecord>
{
	auto const t = (k_ - r.origin().x) / r.direction().x;
	if (t < t_min || t > t_max)
		return std::nullopt;

	auto const y = r.origin().y + t * r.direction().y;
	auto const z = r.origin().z + t * r.direction().z;
	if (y < y0_ || y > y1_ || z < z0_ || z > z1_)
		return std::nullopt;

	auto const u = (y - y0_) / (y1_ - y0_);
	auto const v = (z - z0_) / (z1_ - z0_);
	auto const outward_normal = Vec3{1.0, 0.0, 0.0};
	auto const p = r.at(t);

	return std::make_optional<HitRecord>(
		r,
		p,
		outward_normal,
		t,
		u,
		v,
		material_
	);
}
