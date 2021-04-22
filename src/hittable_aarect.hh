#pragma once

#include <memory>
#include <optional>

#include "hittable.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"

namespace rays::hittable {
	class XyRect;
}

/*
 * Type for an axis-aligned rectangle in the xy-plane.
 */
class rays::hittable::XyRect final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable> new_hittable(double x0, double x1,
			double y0, double y1, double k,
			std::shared_ptr<material::Material> material);

		explicit XyRect(double x0, double x1, double y0, double y1, double k,
			std::shared_ptr<material::Material> material) noexcept;

		std::optional<HitRecord> hit(Ray const &r, double t_min, double t_max)
			const noexcept override;

	private:

		std::shared_ptr<material::Material> material_;
		double x0_, x1_, y0_, y1_, k_;
};
