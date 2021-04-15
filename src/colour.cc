#include "colour.hh"

#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <random>
#include <tuple>

using namespace rays;

/*
 * Creates a random colour.
 */
auto Colour::new_random(std::default_random_engine &rand_eng,
                        double const min,
                        double const max)
	-> Colour
{
	auto rand_dst = std::uniform_real_distribution{min, max};

	auto const r = rand_dst(rand_eng);
	auto const g = rand_dst(rand_eng);
	auto const b = rand_dst(rand_eng);

	return Colour{r, g, b};
}

auto Colour::to_rgb8(int const samples_per_pixel) const noexcept
	-> std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
{
	assert(samples_per_pixel > 0);

	// Divide the colour by the number of samples.
	auto const scale = 1.0 / samples_per_pixel;
	auto [r, g, b] = *this * scale;

	r = std::clamp(r, 0.0, 1.0);
	g = std::clamp(g, 0.0, 1.0);
	b = std::clamp(b, 0.0, 1.0);

	// Gamma-correct for 𝛾 = 2.0.
	r = std::sqrt(r);
	g = std::sqrt(g);
	b = std::sqrt(b);

	auto const ir = static_cast<std::uint8_t>(255.999 * r);
	auto const ig = static_cast<std::uint8_t>(255.999 * g);
	auto const ib = static_cast<std::uint8_t>(255.999 * b);

	return {ir, ig, ib};
}
