#pragma once

#include <cmath>
#include <ostream>
#include <random>

namespace rays {
	struct Vec3;

	std::ostream &operator<<(std::ostream &os, Vec3 v);

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

	constexpr Vec3 reflect(Vec3 v, Vec3 n) noexcept;
	Vec3 refract(Vec3 uv, Vec3 n, double etai_over_etat) noexcept;
}

/*
 * Type for representing vectors in 3-D space.
 */
struct rays::Vec3 final {
	static Vec3 new_random(std::default_random_engine &rand_eng, double min,
		double max);
	static Vec3 new_random_in_unit_sphere(std::default_random_engine &rand_eng);
	static Vec3 new_random_unit(std::default_random_engine &rand_eng);
	static Vec3 new_random_in_hemisphere(std::default_random_engine &rand_eng,
		Vec3 normal);
	static Vec3 new_random_in_unit_disk(std::default_random_engine &rand_eng);

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

	constexpr Vec3 operator-() const noexcept;

	double length() const noexcept;
	Vec3 unit() const noexcept;

	constexpr bool near_zero() const noexcept;
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

inline constexpr auto rays::Vec3::operator-() const noexcept -> Vec3 {
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

/*
 * Returns `true` if the vector is close to zero in all dimensions.
 */
inline constexpr auto rays::Vec3::near_zero() const noexcept -> bool {
	constexpr auto s = 1e-8;
	return std::abs(x) < s && std::abs(y) < s && std::abs(z) < s;
}

/*
 * Reflects `v` from surface with normal `n`.
 */
inline constexpr auto rays::reflect(Vec3 const v, Vec3 const n) noexcept
	-> Vec3
{
	return v - 2.0 * dot(v, n) * n;
}
