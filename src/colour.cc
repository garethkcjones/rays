#include "colour.hh"

#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <tuple>

using namespace rays;

auto Colour::to_rgb8(int const samples_per_pixel) const noexcept
	-> std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
{
	assert(samples_per_pixel > 0);

	// Divide the colour by the number of samples.
	auto const scale = 1.0 / samples_per_pixel;
	auto [r, g, b] = *this * scale;

	// Gamma-correct for 𝛾 = 2.0.
	r = std::sqrt(r);
	g = std::sqrt(g);
	b = std::sqrt(b);

	r = std::clamp(r, 0.0, 0.999);
	g = std::clamp(g, 0.0, 0.999);
	b = std::clamp(b, 0.0, 0.999);

	auto const ir = static_cast<std::uint8_t>(256.0 * r);
	auto const ig = static_cast<std::uint8_t>(256.0 * g);
	auto const ib = static_cast<std::uint8_t>(256.0 * b);

	return {ir, ig, ib};
}
