#pragma once

#include <memory>
#include <random>

#include "colour.hh"
#include "perlin.hh"
#include "texture.hh"
#include "vec3.hh"

namespace rays::texture {
	class Noise;
}

/*
 * Type for representing a random noise texture.
 */
class rays::texture::Noise final:
	public Texture
{
	public:

		static std::shared_ptr<Texture>
			new_texture(std::default_random_engine &rand_eng);

		explicit Noise(std::default_random_engine &rand_eng);

		Colour value(double u, double v, Vec3 p) const override;

	private:

		Perlin noise_;
};
