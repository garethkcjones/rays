#include "material_dielectric.hh"

#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "colour.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"

using namespace rays::material;
using rays::hittable::HitRecord;

Dielectric::Dielectric(double const ir) noexcept:
	ir_{ir}
{
}

auto Dielectric::new_material(double const ir) -> std::shared_ptr<Material> {
	return std::make_shared<Dielectric>(ir);
}

auto Dielectric::scatter(Ray const &r_in,
                         HitRecord const &rec,
                         std::default_random_engine &/*rand_eng*/) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto const attenuation = Colour{1.0, 1.0, 1.0};
	auto const refraction_ratio = rec.front_face() ? (1.0 / ir_) : ir_;

	auto const unit_direction = r_in.direction().unit();
	auto const refracted =
		refract(unit_direction, rec.normal(), refraction_ratio);

	auto const scattered = Ray{rec.p(), refracted};

	return std::make_optional(std::make_pair(attenuation, scattered));
}
