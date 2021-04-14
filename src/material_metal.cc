#include "material_metal.hh"

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

Metal::Metal(Colour const albedo) noexcept:
	albedo_{albedo}
{
}

auto Metal::new_material(Colour const albedo) -> std::shared_ptr<Material> {
	return std::make_shared<Metal>(albedo);
}

auto Metal::scatter(Ray const &r_in,
                    HitRecord const &rec,
                    std::default_random_engine &/*rand_eng*/) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto const reflected = reflect(r_in.direction().unit(), rec.normal());
	auto const attenuation = albedo_;
	auto const scattered = Ray{rec.p(), reflected};

	if (dot(scattered.direction(), rec.normal()) > 0.0)
		return std::make_optional(std::make_pair(attenuation, scattered));
	else
		return std::nullopt;
}
