#pragma once

#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "colour.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "texture.hh"
#include "ray.hh"

namespace rays::material {
	class Isotropic;
}

/*
 * Type for materials that scatter randomly.
 */
class rays::material::Isotropic final:
	public Material
{
	public:

		static std::shared_ptr<Material>
			new_material(std::shared_ptr<texture::Texture> albedo);
		static std::shared_ptr<Material> new_material(Colour albedo);

		explicit Isotropic(std::shared_ptr<texture::Texture> albedo) noexcept;

		std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<texture::Texture> albedo_;
};
