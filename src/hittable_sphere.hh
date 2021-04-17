#pragma once

#include <memory>
#include <optional>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

namespace rays::hittable {
	class Sphere;
	class MovingSphere;
}

/*
 * Type for representing stationary spheres.
 */
class rays::hittable::Sphere final:
	public Hittable
{
	public:
		static std::shared_ptr<Hittable> new_hittable(Vec3 centre,
			double radius, std::shared_ptr<material::Material> material);

		explicit Sphere(Vec3 centre, double radius,
			std::shared_ptr<material::Material> material) noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max)
			const noexcept override;

	private:

		Vec3 centre_;
		double radius_;
		std::shared_ptr<material::Material> material_;
};

/*
 * Type for representing moving spheres.
 */
class rays::hittable::MovingSphere final:
	public Hittable
{
	public:
		static std::shared_ptr<Hittable> new_hittable(Vec3 centre0,
			Vec3 centre1, double time0, double time1, double radius,
			std::shared_ptr<material::Material> material);

		explicit MovingSphere(Vec3 centre0, Vec3 centre1, double time0,
			double time1, double radius,
			std::shared_ptr<material::Material> material) noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max)
			const noexcept override;

	private:

		Vec3 centre0_, centre1_;
		double time0_, time1_;
		double radius_;
		std::shared_ptr<material::Material> material_;

		constexpr Vec3 centre(double time) const noexcept;
};

inline constexpr auto rays::hittable::MovingSphere::centre(double const time)
	const noexcept -> Vec3
{
	return centre0_ + ((time - time0_)
	      / (time1_ - time0_)) * (centre1_ - centre0_);
}
