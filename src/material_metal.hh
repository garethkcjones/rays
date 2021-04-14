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
	class Metal;
}

/*
 * Type for representing a reflective material.
 */
class rays::material::Metal final:
	public Material
{
	public:

		static std::shared_ptr<Material>
			new_material(Colour albedo, double fuzz);

		explicit Metal(Colour albedo, double fuzz) noexcept;

		std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const override;

	private:

		Colour albedo_;
		double fuzz_;
};
