#include "colour.hh"

#include <algorithm>
#include <cstdint>
#include <tuple>

using namespace rays;

auto Colour::to_rgb8() const noexcept
	-> std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
{
	auto const cr = std::clamp(r, 0.0, 1.0);
	auto const cg = std::clamp(g, 0.0, 1.0);
	auto const cb = std::clamp(b, 0.0, 1.0);

	auto const ir = static_cast<std::uint8_t>(255.999 * cr);
	auto const ig = static_cast<std::uint8_t>(255.999 * cg);
	auto const ib = static_cast<std::uint8_t>(255.999 * cb);

	return {ir, ig, ib};
}
