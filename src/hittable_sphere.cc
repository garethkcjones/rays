#include "hittable_sphere.hh"

#include <cassert>
#include <cmath>
#include <memory>
#include <optional>
#include <utility>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;

Sphere::Sphere(Vec3 const centre,
               double const radius,
               std::shared_ptr<material::Material> material) noexcept:
	centre_{centre},
	radius_{radius},
	material_{std::move(material)}
{
	assert(material_);
}

MovingSphere::MovingSphere(Vec3 const centre0,
                           Vec3 const centre1,
                           double const time0,
                           double const time1,
                           double const radius,
                           std::shared_ptr<material::Material> material)
	noexcept:
	centre0_{centre0},
	centre1_{centre1},
	time0_{time0},
	time1_{time1},
	radius_{radius},
	material_{std::move(material)}
{
}

auto Sphere::new_hittable(Vec3 const centre,
                          double const radius,
                          std::shared_ptr<material::Material> material)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<Sphere>(centre, radius, std::move(material));
}

auto MovingSphere::new_hittable(Vec3 const centre0,
                                Vec3 const centre1,
                                double const time0,
                                double const time1,
                                double const radius,
                                std::shared_ptr<material::Material> material)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<MovingSphere>(
		centre0,
		centre1,
		time0,
		time1,
		radius,
		std::move(material)
	);
}

auto Sphere::hit(Ray const &r, double const t_min, double const t_max) const
	noexcept -> std::optional<HitRecord>
{
	auto const oc = r.origin() - centre_;
	auto const a = dot(r.direction(), r.direction());
	auto const half_b = dot(oc, r.direction());
	auto const c = dot(oc, oc) - radius_ * radius_;

	auto const discriminant = half_b * half_b - a * c;
	if (discriminant < 0.0)
		return std::nullopt;

	auto const sqrtd = std::sqrt(discriminant);

	// Find the nearest root that lies in the acceptable range.
	auto root = (-half_b - sqrtd) / a;
	if (root < t_min || t_max < root) {
		root = (-half_b + sqrtd) / a;
		if (root < t_min || t_max < root)
			return std::nullopt;
	}

	auto const t = root;
	auto const p = r.at(t);
	auto const outward_normal = (p - centre_) / radius_;

	return std::make_optional<HitRecord>(r, p, outward_normal, t, material_);
}

auto MovingSphere::hit(Ray const &r, double const t_min, double const t_max)
	const noexcept -> std::optional<HitRecord>
{
	auto const centre = this->centre(r.time());
	auto const oc = r.origin() - centre;
	auto const a = dot(r.direction(), r.direction());
	auto const half_b = dot(oc, r.direction());
	auto const c = dot(oc, oc) - radius_ * radius_;

	auto const discriminant = half_b * half_b - a * c;
	if (discriminant < 0.0)
		return std::nullopt;

	auto const sqrtd = std::sqrt(discriminant);

	// Find the nearest root that lies in the acceptable range.
	auto root = (-half_b - sqrtd) / a;
	if (root < t_min || t_max < root) {
		root = (-half_b + sqrtd) / a;
		if (root < t_min || t_max < root)
			return std::nullopt;
	}

	auto const t = root;
	auto const p = r.at(t);
	auto const outward_normal = (p - centre) / radius_;

	return std::make_optional<HitRecord>(r, p, outward_normal, t, material_);
}
