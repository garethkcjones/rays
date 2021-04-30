#include "hittable_sphere.hh"

#include <cassert>
#include <cmath>
#include <memory>
#include <numbers>
#include <optional>
#include <random>
#include <utility>

#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;

namespace {
	auto get_sphere_uv(rays::Vec3 const p) noexcept -> std::pair<double, double>
	{
		// p: a given point on the sphere of radius one, centred at the origin.
		// u: returned value [0,1] of angle around the Y axis from X=-1.
		// v: returned value [0,1] of angle from Y=-1 to Y=+1.
		//     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
		//     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
		//     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

		auto const theta = std::acos(-p.y);
		auto const phi = std::atan2(-p.z, p.x) + std::numbers::pi;

		auto const u = 0.5 * phi * std::numbers::inv_pi;
		auto const v = theta * std::numbers::inv_pi;

		return std::make_pair(u, v);
	}
}

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

auto Sphere::hit(Ray const &r,
                 double const t_min,
                 double const t_max,
                 std::default_random_engine &/*rand_eng*/) const
	-> std::optional<HitRecord>
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
	auto const [u, v] = get_sphere_uv(outward_normal);

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

auto MovingSphere::hit(Ray const &r,
                       double const t_min,
                       double const t_max,
                       std::default_random_engine &/*rand_eng*/) const
	-> std::optional<HitRecord>
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
	auto const [u, v] = get_sphere_uv(outward_normal);

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

auto Sphere::bounding_box(double /*time0*/, double /*time1*/) const -> Aabb {
	auto const radius = Vec3{radius_, radius_, radius_};
	auto const minimum = centre_ - radius;
	auto const maximum = centre_ + radius;
	return Aabb{minimum, maximum};
}

auto MovingSphere::bounding_box(double const time0, double const time1) const
	-> Aabb
{
	auto const radius = Vec3{radius_, radius_, radius_};

	auto const centre0 = centre(time0);
	auto const minimum0 = centre0 - radius;
	auto const maximum0 = centre0 + radius;
	auto const box0 = Aabb{minimum0, maximum0};

	auto const centre1 = centre(time1);
	auto const minimum1 = centre1 - radius;
	auto const maximum1 = centre1 + radius;
	auto const box1 = Aabb{minimum1, maximum1};

	return Aabb::surrounding_box(box0, box1);
}
