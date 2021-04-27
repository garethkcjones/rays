#pragma once

#include <memory>
#include <optional>
#include <random>

#include "colour.hh"
#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "texture.hh"

namespace rays::hittable {
	class ConstantMedium;
}

/*
 * Type for an isotropic medium.
 */
class rays::hittable::ConstantMedium final:
	public Hittable
{
	public:

		static std::shared_ptr<Hittable>
			new_hittable(std::shared_ptr<Hittable> boundary, double density,
			std::shared_ptr<texture::Texture> albedo);

		static std::shared_ptr<Hittable>
			new_hittable(std::shared_ptr<Hittable> boundary, double density,
			Colour albedo);

		explicit ConstantMedium(std::shared_ptr<Hittable> boundary,
			double density, std::shared_ptr<texture::Texture> albedo);

		std::optional<HitRecord> hit(Ray const &r, double t_min,
			double t_max, std::default_random_engine &rand_eng) const override;

		Aabb bounding_box(double time0, double time1) const override;

	private:

		std::shared_ptr<Hittable> boundary_;
		std::shared_ptr<material::Material> phase_function_;
		double neg_inv_density_;
};
