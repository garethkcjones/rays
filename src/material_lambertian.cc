#include "material_lambertian.hh"

#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::material;
using rays::hittable::HitRecord;

Lambertian0::Lambertian0(Colour const albedo) noexcept:
	albedo_{albedo}
{
}

Lambertian1::Lambertian1(Colour const albedo) noexcept:
	albedo_{albedo}
{
}

Lambertian2::Lambertian2(Colour const albedo) noexcept:
	albedo_{albedo}
{
}

auto Lambertian0::new_material(Colour const albedo) -> std::shared_ptr<Material>
{
	return std::make_shared<Lambertian0>(albedo);
}

auto Lambertian1::new_material(Colour const albedo) -> std::shared_ptr<Material>
{
	return std::make_shared<Lambertian1>(albedo);
}

auto Lambertian2::new_material(Colour const albedo) -> std::shared_ptr<Material>
{
	return std::make_shared<Lambertian2>(albedo);
}

auto Lambertian0::scatter(Ray const &/*r_in*/,
                          HitRecord const &rec,
                          std::default_random_engine &rand_eng) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto scatter_direction =
		Vec3::new_random_in_hemisphere(rand_eng, rec.normal());

	// Catch degenerate scatter direction.
	if (scatter_direction.near_zero())
		scatter_direction = rec.normal();

	auto const attenuation = albedo_;
	auto const scattered = Ray{rec.p(), scatter_direction};

	return std::make_optional(std::make_pair(attenuation, scattered));
}

auto Lambertian1::scatter(Ray const &/*r_in*/,
                          HitRecord const &rec,
                          std::default_random_engine &rand_eng) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto scatter_direction =
		rec.normal() + Vec3::new_random_in_unit_sphere(rand_eng);

	// Catch degenerate scatter direction.
	if (scatter_direction.near_zero())
		scatter_direction = rec.normal();

	auto const attenuation = albedo_;
	auto const scattered = Ray{rec.p(), scatter_direction};

	return std::make_optional(std::make_pair(attenuation, scattered));
}

auto Lambertian2::scatter(Ray const &/*r_in*/,
                          HitRecord const &rec,
                          std::default_random_engine &rand_eng) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto scatter_direction = rec.normal() + Vec3::new_random_unit(rand_eng);

	// Catch degenerate scatter direction.
	if (scatter_direction.near_zero())
		scatter_direction = rec.normal();

	auto const attenuation = albedo_;
	auto const scattered = Ray{rec.p(), scatter_direction};

	return std::make_optional(std::make_pair(attenuation, scattered));
}
