#pragma once

#include <cstdint>
#include <random>
#include <tuple>

namespace rays {
	struct Colour;

	constexpr Colour operator+(Colour c1, Colour c2) noexcept;
	constexpr Colour operator*(Colour c1, Colour c2) noexcept;

	constexpr Colour operator*(Colour c, double s) noexcept;
	constexpr Colour operator/(Colour c, double s) noexcept;

	constexpr Colour operator*(double s, Colour c) noexcept;
}

/*
 * Type for representing colours.
 */
struct rays::Colour final {
	static Colour new_random(std::default_random_engine &rand_eng, double min,
		double max);

	double r = 0.0;
	double g = 0.0;
	double b = 0.0;

	constexpr Colour &operator+=(Colour c) noexcept;
	constexpr Colour &operator*=(Colour c) noexcept;

	constexpr Colour &operator*=(double s) noexcept;
	constexpr Colour &operator/=(double s) noexcept;

	std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
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

inline constexpr auto rays::Colour::operator*=(Colour const c) noexcept
	-> Colour &
{
	r *= c.r;
	g *= c.g;
	b *= c.b;
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

inline constexpr auto rays::operator*(Colour c1, Colour const c2) noexcept
	-> Colour
{
	return c1 *= c2;
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
