#pragma once

#include <cmath>

namespace rays {
	struct Vec3;

	constexpr Vec3 operator+(Vec3 v1, Vec3 v2) noexcept;
	constexpr Vec3 operator-(Vec3 v1, Vec3 v2) noexcept;
	constexpr Vec3 operator*(Vec3 v1, Vec3 v2) noexcept;
	constexpr Vec3 operator/(Vec3 v1, Vec3 v2) noexcept;

	constexpr Vec3 operator+(Vec3 v, double s) noexcept;
	constexpr Vec3 operator-(Vec3 v, double s) noexcept;
	constexpr Vec3 operator*(Vec3 v, double s) noexcept;
	constexpr Vec3 operator/(Vec3 v, double s) noexcept;

	constexpr Vec3 operator+(double s, Vec3 v) noexcept;
	constexpr Vec3 operator-(double s, Vec3 v) noexcept;
	constexpr Vec3 operator*(double s, Vec3 v) noexcept;
	constexpr Vec3 operator/(double s, Vec3 v) noexcept;

	constexpr Vec3 cross(Vec3 v1, Vec3 v2) noexcept;
	constexpr double dot(Vec3 v1, Vec3 v2) noexcept;
}

/*
 * Type for representing vectors in 3-D space.
 */
struct rays::Vec3 final {
	double x = 0.0;
	double y = 0.0;
	double z = 0.0;

	constexpr Vec3 &operator+=(Vec3 v) noexcept;
	constexpr Vec3 &operator-=(Vec3 v) noexcept;
	constexpr Vec3 &operator*=(Vec3 v) noexcept;
	constexpr Vec3 &operator/=(Vec3 v) noexcept;

	constexpr Vec3 &operator+=(double s) noexcept;
	constexpr Vec3 &operator-=(double s) noexcept;
	constexpr Vec3 &operator*=(double s) noexcept;
	constexpr Vec3 &operator/=(double s) noexcept;

	constexpr Vec3 operator-() noexcept;

	double length() const noexcept;
	Vec3 unit() const noexcept;
};

inline constexpr auto rays::Vec3::operator+=(Vec3 const v) noexcept -> Vec3 & {
	x += v.x;
	y += v.y;
	z += v.z;
	return *this;
}

inline constexpr auto rays::Vec3::operator-=(Vec3 const v) noexcept -> Vec3 & {
	x -= v.x;
	y -= v.y;
	z -= v.z;
	return *this;
}

inline constexpr auto rays::Vec3::operator*=(Vec3 const v) noexcept -> Vec3 & {
	x *= v.x;
	y *= v.y;
	z *= v.z;
	return *this;
}

inline constexpr auto rays::Vec3::operator/=(Vec3 const v) noexcept -> Vec3 & {
	x /= v.x;
	y /= v.y;
	z /= v.z;
	return *this;
}

inline constexpr auto rays::Vec3::operator+=(double const s) noexcept -> Vec3 &
{
	x += s;
	y += s;
	z += s;
	return *this;
}

inline constexpr auto rays::Vec3::operator-=(double const s) noexcept -> Vec3 &
{
	x -= s;
	y -= s;
	z -= s;
	return *this;
}

inline constexpr auto rays::Vec3::operator*=(double const s) noexcept -> Vec3 &
{
	x *= s;
	y *= s;
	z *= s;
	return *this;
}

inline constexpr auto rays::Vec3::operator/=(double const s) noexcept -> Vec3 &
{
	x /= s;
	y /= s;
	z /= s;
	return *this;
}

inline constexpr auto rays::Vec3::operator-() noexcept -> Vec3 {
	return {-x, -y, -z};
}

inline constexpr auto rays::operator+(Vec3 v1, Vec3 const v2) noexcept -> Vec3 {
	return v1 += v2;
}

inline constexpr auto rays::operator-(Vec3 v1, Vec3 const v2) noexcept -> Vec3 {
	return v1 -= v2;
}

inline constexpr auto rays::operator*(Vec3 v1, Vec3 const v2) noexcept -> Vec3 {
	return v1 *= v2;
}

inline constexpr auto rays::operator/(Vec3 v1, Vec3 const v2) noexcept -> Vec3 {
	return v1 /= v2;
}

inline constexpr auto rays::operator+(Vec3 v, double const s) noexcept -> Vec3 {
	return v += s;
}

inline constexpr auto rays::operator-(Vec3 v, double const s) noexcept -> Vec3 {
	return v -= s;
}

inline constexpr auto rays::operator*(Vec3 v, double const s) noexcept -> Vec3 {
	return v *= s;
}

inline constexpr auto rays::operator/(Vec3 v, double const s) noexcept -> Vec3 {
	return v /= s;
}

inline constexpr auto rays::operator+(double const s, Vec3 const v) noexcept
	-> Vec3
{
	auto const x = s + v.x;
	auto const y = s + v.y;
	auto const z = s + v.z;
	return {x, y, z};
}

inline constexpr auto rays::operator-(double const s, Vec3 const v) noexcept
	-> Vec3
{
	auto const x = s - v.x;
	auto const y = s - v.y;
	auto const z = s - v.z;
	return {x, y, z};
}

inline constexpr auto rays::operator*(double const s, Vec3 const v) noexcept
	-> Vec3
{
	auto const x = s * v.x;
	auto const y = s * v.y;
	auto const z = s * v.z;
	return {x, y, z};
}

inline constexpr auto rays::operator/(double const s, Vec3 const v) noexcept
	-> Vec3
{
	auto const x = s / v.x;
	auto const y = s / v.y;
	auto const z = s / v.z;
	return {x, y, z};
}

inline constexpr auto rays::cross(Vec3 const v1, Vec3 const v2) noexcept -> Vec3
{
	auto const x = v1.y * v2.z - v1.z * v2.y;
	auto const y = v1.z * v2.x - v1.x * v2.z;
	auto const z = v1.x * v2.y - v1.y * v2.x;
	return {x, y, z};
}

inline constexpr auto rays::dot(Vec3 v1, Vec3 const v2) noexcept -> double {
	v1 *= v2;
	return v1.x + v1.y + v1.z;
}

inline auto rays::Vec3::length() const noexcept -> double {
	return std::hypot(x, y, z);
}

inline auto rays::Vec3::unit() const noexcept -> Vec3 {
	return *this / length();
}
