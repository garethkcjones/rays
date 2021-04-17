#include "material_dielectric.hh"

#include <algorithm>
#include <cmath>
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

namespace {
	/*
	 * Use Schlick’s approximation for reflectance.
	 */
	constexpr auto reflectance(double const cosine, double const ref_idx)
		noexcept -> double
	{
		auto r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		return r0 + (1.0 - r0) * std::pow((1.0 - cosine), 5);
	}
}

Dielectric::Dielectric(double const ir) noexcept:
	ir_{ir}
{
}

auto Dielectric::new_material(double const ir) -> std::shared_ptr<Material> {
	return std::make_shared<Dielectric>(ir);
}

auto Dielectric::scatter(Ray const &r_in,
                         HitRecord const &rec,
                         std::default_random_engine &rand_eng) const
	-> std::optional<std::pair<Colour, Ray>>
{
	auto rand_dst = std::uniform_real_distribution{0.0, 1.0};

	auto const attenuation = Colour{1.0, 1.0, 1.0};
	auto const refraction_ratio = rec.front_face() ? (1.0 / ir_) : ir_;

	auto const unit_direction = r_in.direction().unit();
	auto const cos_theta =
		std::clamp(dot(-unit_direction, rec.normal()), -1.0, 1.0);
	auto const sin_theta = std::sqrt(1.0 - cos_theta * cos_theta);

	auto const cannot_refract = refraction_ratio * sin_theta > 1.0;
	auto const direction = (cannot_refract
	          || reflectance(cos_theta, refraction_ratio) > rand_dst(rand_eng))
		? reflect(unit_direction, rec.normal())
		: refract(unit_direction, rec.normal(), refraction_ratio);

	auto const scattered = Ray{rec.p(), direction, r_in.time()};

	return std::make_optional(std::make_pair(attenuation, scattered));
}
