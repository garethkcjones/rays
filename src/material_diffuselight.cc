#include "material_diffuselight.hh"

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

DiffuseLight::DiffuseLight(std::shared_ptr<Texture> emit) noexcept:
	emit_{std::move(emit)}
{
	assert(emit_);
}

auto DiffuseLight::new_material(std::shared_ptr<Texture> emit)
	-> std::shared_ptr<Material>
{
	return std::make_shared<DiffuseLight>(std::move(emit));
}

auto DiffuseLight::new_material(Colour const emit) -> std::shared_ptr<Material>
{
	return new_material(SolidColour::new_texture(emit));
}

auto DiffuseLight::scatter(Ray const &/*r_in*/,
                           HitRecord const &/*rec*/,
                           std::default_random_engine &/*rand_eng*/) const
	-> std::optional<std::pair<Colour, Ray>>
{
	return std::nullopt;
}

auto DiffuseLight::emitted(double const u, double const v, Vec3 const p) const
	-> Colour
{
	return emit_->value(u, v, p);
}
