#pragma once

#include <memory>

#include "colour.hh"
#include "texture.hh"
#include "vec3.hh"

namespace rays::texture {
	class Chequer;
}

/*
 * Type for representing a chequered texture.
 */
class rays::texture::Chequer final:
	public Texture
{
	public:

		static std::shared_ptr<Texture>
			new_texture(std::shared_ptr<Texture> even,
			std::shared_ptr<Texture> odd);

		static std::shared_ptr<Texture> new_texture(Colour even, Colour odd);

		explicit Chequer(std::shared_ptr<Texture> even,
			std::shared_ptr<Texture> odd) noexcept;

		explicit Chequer(Colour even, Colour odd);

		Colour value(double u, double v, Vec3 p) const override;

	private:

		std::shared_ptr<Texture> even_, odd_;
};
