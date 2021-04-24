#include "material_isotropic.hh"

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

Isotropic::Isotropic(std::shared_ptr<Texture> albedo) noexcept:
	albedo_{std::move(albedo)}
{
	assert(albedo_);
}

auto Isotropic::new_material(std::shared_ptr<Texture> albedo)
	-> std::shared_ptr<Material>
{
	return std::make_shared<Isotropic>(std::move(albedo));
}

auto Isotropic::new_material(Colour const albedo) -> std::shared_ptr<Material> {
	return new_material(SolidColour::new_texture(albedo));
}

auto Isotropic::scatter(Ray const &r_in,
                        HitRecord const &rec,
                        std::default_random_engine &rand_eng) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto const scattered =
		Ray{rec.p(), Vec3::new_random_in_unit_sphere(rand_eng), r_in.time()};
	auto const attenuation = albedo_->value(rec.u(), rec.v(), rec.p());
	return std::make_optional(std::make_pair(attenuation, scattered));
}
