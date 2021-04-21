#pragma once

#include <optional>
#include <random>
#include <utility>

#include "colour.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"
#include "vec3.hh"

namespace rays::material {
	class Material;
}

/*
 * Abstract type for materials.
 */
class rays::material::Material {
	public:

		virtual ~Material() noexcept = default;

		virtual std::optional<std::pair<Colour, Ray>> scatter(Ray const &r_in,
			hittable::HitRecord const &rec,
			std::default_random_engine &rand_eng) const = 0;

		virtual Colour emitted(double u, double v, Vec3 p) const;
};

inline auto rays::material::Material::emitted(double /*u*/,
                                              double /*v*/,
                                              Vec3 /*p*/) const
	-> Colour
{
	return Colour{0.0, 0.0, 0.0};
}
