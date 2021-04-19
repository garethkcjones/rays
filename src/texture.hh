#pragma once

#include "colour.hh"
#include "vec3.hh"

namespace rays::texture {
	class Texture;
}

/*
 * Abstract type for textures.
 */
class rays::texture::Texture {
	public:

		virtual ~Texture() noexcept = default;

		virtual Colour value(double u, double v, Vec3 p) const = 0;
};
