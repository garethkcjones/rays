#include "hittable_sphere.hh"

#include <cmath>
#include <memory>
#include <optional>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;

Sphere::Sphere(Vec3 const centre, double const radius) noexcept:
	centre_{centre},
	radius_{radius}
{
}

auto Sphere::new_hittable(Vec3 const centre, double const radius)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<Sphere>(centre, radius);
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
	auto const normal = (p - centre_) / radius_;

	return std::make_optional<HitRecord>(p, normal, t);
}
