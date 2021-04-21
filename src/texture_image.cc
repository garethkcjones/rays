#include "texture_image.hh"

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <filesystem>
#include <memory>
#include <stdexcept>
#include <utility>

#include "colour.hh"
#include "texture.hh"
#include "vec3.hh"

// Disable pedantic warnings for this external library.
#if defined(_MSC_VER)
	// Microsoft Visual C++ Compiler
	#pragma warning (push, 0)
#elif defined(__GNUC__)
	// GCC
	#pragma GCC diagnostic push
	#pragma GCC diagnostic ignored "-Wsign-compare"
	#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif

#define STB_IMAGE_IMPLEMENTATION
#include "stb/stb_image.h"

// Restore warning levels.
#if defined(_MSC_VER)
	// Microsoft Visual C++ Compiler
	#pragma warning (pop)
#elif defined(__GNUC__)
	#pragma GCC diagnostic pop
#endif

using namespace rays::texture;

namespace {
	constexpr std::size_t bytes_per_pixel = 3;
}

Image::Image(
	std::unique_ptr<unsigned char[], void (*)(void *)> data,
	std::size_t const width,
	std::size_t const height,
	std::size_t const bytes_per_scanline):
	data_{std::move(data)},
	width_{width},
	height_{height},
	bytes_per_scanline_{bytes_per_scanline}
{
	assert(data);
}

auto Image::new_texture(std::filesystem::path const &filename)
	-> std::shared_ptr<Texture>
{
	auto components_per_pixel = static_cast<int>(bytes_per_pixel);

	int width, height;
	auto data = std::unique_ptr<unsigned char[], void (*)(void *)> {
		stbi_load(
			filename.string().c_str(),
			&width,
			&height,
			&components_per_pixel,
			components_per_pixel
		),
		&stbi_image_free
	};

	if (!data) {
		throw std::runtime_error {
			"cannot load texture image file “" + filename.string() + "”"
		};
	}

	assert(width > 0);
	assert(height > 0);
	assert(components_per_pixel > 0);

	auto const bytes_per_scanline = bytes_per_pixel * width;

	return std::make_shared<Image>(
		std::move(data),
		width,
		height,
		bytes_per_scanline
	);
}

auto Image::value(double u, double v, Vec3 /*p*/) const -> Colour {
	// Clamp input texture coordinates to [0,1] x [1,0].
	u = std::clamp(u, 0.0, 1.0);
	v = 1.0 - std::clamp(v, 0.0, 1.0);  // Flip V to image coordinates.

	auto i = static_cast<std::size_t>(u * width_);
	auto j = static_cast<std::size_t>(v * height_);

	// Clamp integer mapping, since actual coordinates should be less than 1.0.
	i = std::min(i, width_ - 1);
	j = std::min(j, height_ - 1);

	constexpr auto colour_scale = 1.0 / 255.0;

	auto const ind = j * bytes_per_scanline_ + i * bytes_per_pixel;
	assert(ind + 2 < width_ * height_ * bytes_per_pixel);
	auto const pixelr = data_[ind];
	auto const pixelg = data_[ind + 1];
	auto const pixelb = data_[ind + 2];
	auto const pixel = Colour {
		static_cast<double>(pixelr),
		static_cast<double>(pixelg),
		static_cast<double>(pixelb)
	};

	return colour_scale * pixel;
}
