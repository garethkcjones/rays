#pragma once

#include <cstddef>
#include <filesystem>
#include <memory>

#include "colour.hh"
#include "texture.hh"
#include "vec3.hh"

namespace rays::texture {
	class Image;
}

/*
 * Type for image textures.
 */
class rays::texture::Image final:
	public Texture
{
	public:

		static std::shared_ptr<Texture>
			new_texture(std::filesystem::path const &filename);

		explicit Image(std::unique_ptr<unsigned char[], void (*)(void *)> data,
			std::size_t width, std::size_t height,
			std::size_t bytes_per_scanline);

		Colour value(double u, double v, Vec3 p) const override;

	private:

		std::unique_ptr<unsigned char[], void (*)(void *)> data_;
		std::size_t width_, height_;
		std::size_t bytes_per_scanline_;
};
