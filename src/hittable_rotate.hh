#pragma once

#include <memory>
#include <optional>
#include <random>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"

namespace rays::hittable {
	class RotateX;
	class RotateY;
	class RotateZ;
}

/*
 * Wrapper for rotating hittable objects about the x-axis.
 */
class rays::hittable::RotateX final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable>
			new_hittable(std::shared_ptr<Hittable> object, double angle);

		explicit RotateX(std::shared_ptr<Hittable> object, double theta)
			noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<Hittable> object_;
		double sin_theta_, cos_theta_;
};

/*
 * Wrapper for rotating hittable objects about the y-axis.
 */
class rays::hittable::RotateY final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable>
			new_hittable(std::shared_ptr<Hittable> object, double angle);

		explicit RotateY(std::shared_ptr<Hittable> object, double theta)
			noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<Hittable> object_;
		double sin_theta_, cos_theta_;
};

/*
 * Wrapper for rotating hittable objects about the z-axis.
 */
class rays::hittable::RotateZ final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable>
			new_hittable(std::shared_ptr<Hittable> object, double angle);

		explicit RotateZ(std::shared_ptr<Hittable> object, double theta)
			noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<Hittable> object_;
		double sin_theta_, cos_theta_;
};
