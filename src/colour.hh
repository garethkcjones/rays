#pragma once

#include <cstdint>
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
	double r = 0.0;
	double g = 0.0;
	double b = 0.0;

	constexpr Colour &operator+=(Colour c) noexcept;
	constexpr Colour &operator*=(Colour c) noexcept;

	constexpr Colour &operator*=(double s) noexcept;
	constexpr Colour &operator/=(double s) noexcept;

	constexpr std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
		to_rgb8() const noexcept;
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

inline constexpr auto rays::Colour::to_rgb8() const noexcept
	-> std::tuple<std::uint8_t, std::uint8_t, std::uint8_t>
{
	auto const ir = static_cast<std::uint8_t>(255.999 * r);
	auto const ig = static_cast<std::uint8_t>(255.999 * g);
	auto const ib = static_cast<std::uint8_t>(255.999 * b);
	return {ir, ig, ib};
}
