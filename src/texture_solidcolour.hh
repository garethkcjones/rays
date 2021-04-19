#pragma once

#include "colour.hh"
#include "texture.hh"
#include "vec3.hh"

#include <memory>

namespace rays::texture {
	class SolidColour;
}

/*
 * Type for representing solid colour textures.
 */
class rays::texture::SolidColour final:
	public Texture
{
	public:

		static std::shared_ptr<Texture> new_texture(Colour value);

		SolidColour(Colour value) noexcept;

		Colour value(double u, double v, Vec3 p) const override;

	private:

		Colour value_;
};
