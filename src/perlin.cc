#include "perlin.hh"

#include <array>
#include <cassert>
#include <cstddef>
#include <numeric>
#include <random>
#include <utility>

#include "vec3.hh"

using namespace rays;

namespace {
	void permute(std::array<std::size_t, Perlin::point_count> &p,
	             std::default_random_engine &rand_eng)
	{
		using std::swap;

		for (auto i = p.size() - 1; i > 0; --i) {
			auto rand_dst = std::uniform_int_distribution<std::size_t>{0, i};
			auto const target = rand_dst(rand_eng);
			assert(target < p.size());
			swap(p[i], p[target]);
		}
	}

	auto perlin_generate_perm(std::default_random_engine &rand_eng)
		-> std::array<std::size_t, Perlin::point_count>
	{
		using std::begin;
		using std::end;

		std::array<std::size_t, Perlin::point_count> p;
		std::iota(begin(p), end(p), 0);
		permute(p, rand_eng);
		return p;
	}

	auto rand_fill(std::default_random_engine &rand_eng)
		-> std::array<double, Perlin::point_count>
	{
		std::array<double, Perlin::point_count> ranfloat;
		auto rand_dst = std::uniform_real_distribution{0.0, 1.0};
		for (auto &&i : ranfloat)
			i = rand_dst(rand_eng);
		return ranfloat;
	}
}

Perlin::Perlin(std::default_random_engine &rand_eng):
	ranfloat_{rand_fill(rand_eng)},
	perm_x_{perlin_generate_perm(rand_eng)},
	perm_y_{perlin_generate_perm(rand_eng)},
	perm_z_{perlin_generate_perm(rand_eng)}
{
}

auto Perlin::noise(Vec3 const p) const noexcept -> double {
	constexpr auto mask = Perlin::point_count - 1;

	auto const i = static_cast<std::size_t>(4.0 * p.x) & mask;
	auto const j = static_cast<std::size_t>(4.0 * p.y) & mask;
	auto const k = static_cast<std::size_t>(4.0 * p.z) & mask;

	assert(i < perm_x_.size());
	assert(j < perm_y_.size());
	assert(k < perm_z_.size());

	auto const ind = perm_x_[i] ^ perm_y_[j] ^ perm_z_[k];
	assert(ind < ranfloat_.size());
	return ranfloat_[ind];
}
