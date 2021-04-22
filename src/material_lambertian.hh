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
	class Lambertian0;
	class Lambertian1;
	class Lambertian2;
}

/*
 * Type for representing a pre-Lambertian scattering material.
 */
class rays::material::Lambertian0 final:
	public Material
{
	public:

		static std::shared_ptr<Material>
			new_material(std::shared_ptr<texture::Texture> albedo);
		static std::shared_ptr<Material> new_material(Colour albedo);

		explicit Lambertian0(std::shared_ptr<texture::Texture> albedo) noexcept;

		std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<texture::Texture> albedo_;
};

/*
 * Type for representing a pseudo-Lambertian scattering material.
 */
class rays::material::Lambertian1 final:
	public Material
{
	public:

		static std::shared_ptr<Material>
			new_material(std::shared_ptr<texture::Texture> albedo);
		static std::shared_ptr<Material> new_material(Colour albedo);

		explicit Lambertian1(std::shared_ptr<texture::Texture> albedo) noexcept;

		std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<texture::Texture> albedo_;
};

/*
 * Type for representing a true Lambertian scattering material.
 */
class rays::material::Lambertian2 final:
	public Material
{
	public:

		static std::shared_ptr<Material>
			new_material(std::shared_ptr<texture::Texture> albedo);
		static std::shared_ptr<Material> new_material(Colour albedo);

		explicit Lambertian2(std::shared_ptr<texture::Texture> albedo) noexcept;

		std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const override;

	private:

		std::shared_ptr<texture::Texture> albedo_;
};
