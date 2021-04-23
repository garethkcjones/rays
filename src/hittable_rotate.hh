#pragma once

#include <memory>
#include <optional>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"

namespace rays::hittable {
	class RotateY;
}

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

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max)
			const noexcept override;

	private:

		std::shared_ptr<Hittable> object_;
		double sin_theta_, cos_theta_;
};
