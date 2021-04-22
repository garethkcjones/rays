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

		static std::shared_ptr<Texture> new_texture(Vec3 scale,
			std::shared_ptr<Texture> even, std::shared_ptr<Texture> odd);

		static std::shared_ptr<Texture> new_texture(Vec3 scale, Colour even,
			Colour odd);

		explicit Chequer(Vec3 scale, std::shared_ptr<Texture> even,
			std::shared_ptr<Texture> odd) noexcept;

		Colour value(double u, double v, Vec3 p) const override;

	private:

		Vec3 scale_;
		std::shared_ptr<Texture> even_, odd_;
};
