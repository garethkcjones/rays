#pragma once

#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "colour.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "texture.hh"

namespace rays::material {
	class DiffuseLight;
}

/*
 * Type for materials emitting difuse light.
 */
class rays::material::DiffuseLight final:
	public Material
{
	public:

		static std::shared_ptr<Material>
			new_material(std::shared_ptr<texture::Texture> emit);
		static std::shared_ptr<Material> new_material(Colour emit);

		explicit DiffuseLight(std::shared_ptr<texture::Texture> emit) noexcept;
		explicit DiffuseLight(Colour emit);

		std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const override;

		Colour emitted(double u, double v, Vec3 p) const override;

	private:

		std::shared_ptr<texture::Texture> emit_;
};
