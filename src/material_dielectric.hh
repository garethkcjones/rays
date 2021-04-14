#pragma once

#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "colour.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"

namespace rays::material {
	class Dielectric;
}

/*
 * Type for representing a transparent material.
 */
class rays::material::Dielectric final:
	public Material
{
	public:

		static std::shared_ptr<Material> new_material(double ir);

		explicit Dielectric(double ir) noexcept;

		std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const override;

	private:

		double ir_; // Index of refraction.
};
