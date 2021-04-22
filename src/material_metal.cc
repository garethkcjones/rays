#include "material_metal.hh"

#include <cassert>
#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "colour.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "texture.hh"
#include "texture_solidcolour.hh"
#include "vec3.hh"

using namespace rays::material;
using rays::hittable::HitRecord;
using rays::texture::SolidColour;
using rays::texture::Texture;

Metal::Metal(std::shared_ptr<Texture> albedo, double const fuzz) noexcept:
	albedo_{std::move(albedo)},
	fuzz_{fuzz}
{
	assert(albedo_);
	assert(fuzz_ >= 0.0);
	assert(fuzz_ <= 1.0);
}

auto Metal::new_material(std::shared_ptr<Texture> albedo, double const fuzz)
	-> std::shared_ptr<Material>
{
	return std::make_shared<Metal>(albedo, fuzz);
}

auto Metal::new_material(Colour const albedo, double const fuzz)
	-> std::shared_ptr<Material>
{
	return new_material(SolidColour::new_texture(albedo), fuzz);
}

auto Metal::scatter(Ray const &r_in,
                    HitRecord const &rec,
                    std::default_random_engine &rand_eng) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto const reflected = reflect(r_in.direction().unit(), rec.normal());
	auto const attenuation = albedo_->value(rec.u(), rec.v(), rec.p());
	auto const scattered = Ray {
		rec.p(),
		reflected + fuzz_ * Vec3::new_random_in_unit_sphere (rand_eng),
		r_in.time()
	};

	if (dot(scattered.direction(), rec.normal()) > 0.0)
		return std::make_optional(std::make_pair(attenuation, scattered));
	else
		return std::nullopt;
}
