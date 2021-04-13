#pragma once

#include <optional>
#include <utility>

#include "colour.hh"
#include "hittable_hitrecord.hh"
#include "ray.hh"

namespace rays::material {
	class Material;
}

/*
 * Abstract type for materials.
 */
class rays::material::Material {
	public:

		virtual ~Material() noexcept = default;

		virtual std::optional<std::pair<Colour, Ray>>
			scatter(Ray const &r_in, hittable::HitRecord const &rec) const = 0;
};