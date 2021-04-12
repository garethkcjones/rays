#pragma once

#include <cassert>
#include <algorithm>
#include <cstdint>
#include <tuple>

namespace rays {
	struct Colour;

	constexpr Colour operator+(Colour c1, Colour c2) noexcept;

	constexpr Colour operator*(Colour c, double s) noexcept;
	constexpr Colour operator/(Colour c, double s) noexcept;

	constexpr Colour operator*(double s, Colour c) noexcept;
}

/*
 * Type for representing colours.
 */
struct rays::Colour final {
	double r = 0.0;
	double g = 0.0;
	double b = 0.0;

	constexpr Colour &operator+=(Colour c) noexcept;

	constexpr Colour &operator*=(double s) noexcept;
	constexpr Colour &operator/=(double s) noexcept;

	constexpr std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
		to_rgb8(int samples_per_pixel) const noexcept;
};

inline constexpr auto rays::Colour::operator+=(Colour const c) noexcept
	-> Colour &
{
	r += c.r;
	g += c.g;
	b += c.b;
	return *this;
}

inline constexpr auto rays::Colour::operator*=(double const s) noexcept
	-> Colour &
{
	r *= s;
	g *= s;
	b *= s;
	return *this;
}

inline constexpr auto rays::Colour::operator/=(double const s) noexcept
	-> Colour &
{
	r /= s;
	g /= s;
	b /= s;
	return *this;
}

inline constexpr auto rays::operator+(Colour c1, Colour const c2) noexcept
	-> Colour
{
	return c1 += c2;
}

inline constexpr auto rays::operator*(Colour c, double const s) noexcept
	-> Colour
{
	return c *= s;
}

inline constexpr auto rays::operator/(Colour c, double const s) noexcept
	-> Colour
{
	return c /= s;
}

inline constexpr auto rays::operator*(double const s, Colour const c) noexcept
	-> Colour
{
	auto const r = s * c.r;
	auto const g = s * c.g;
	auto const b = s * c.b;
	return {r, g, b};
}

inline constexpr auto rays::Colour::to_rgb8(int const samples_per_pixel) const
	noexcept -> std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
{
	assert(samples_per_pixel > 0);

	// Divide the colour by the number of samples.
	auto const scale = 1.0 / samples_per_pixel;
	auto [r, g, b] = *this * scale;

	r = std::clamp(r, 0.0, 0.999);
	g = std::clamp(g, 0.0, 0.999);
	b = std::clamp(b, 0.0, 0.999);

	auto const ir = static_cast<std::uint8_t>(256.0 * r);
	auto const ig = static_cast<std::uint8_t>(256.0 * g);
	auto const ib = static_cast<std::uint8_t>(256.0 * b);

	return {ir, ig, ib};
}
