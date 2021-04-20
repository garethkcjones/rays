#include "perlin.hh"

#include <array>
#include <cassert>
#include <cmath>
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
		-> std::array<Vec3, Perlin::point_count>
	{
		std::array<Vec3, Perlin::point_count> ranvec;
		for (auto &&i : ranvec)
			i = Vec3::new_random(rand_eng, -1.0, 1.0);
		return ranvec;
	}

	auto perlin_interp(
		std::array<std::array<std::array<Vec3, 2>, 2>, 2> const &c,
		double const u,
		double const v,
		double const w
	) -> double {
		auto const uu = u * u * (3.0 - 2.0 * u);
		auto const vv = v * v * (3.0 - 2.0 * v);
		auto const ww = w * w * (3.0 - 2.0 * w);

		auto accum = 0.0;
		for (auto i = 0; i < 2; ++i) {
			auto const iterm = i * uu + (1.0 - i) * (1.0 - uu);
			auto const iweight = u - i;

			for (auto j = 0; j < 2; ++j) {
				auto const jterm = j * vv + (1.0 - j) * (1.0 - vv);
				auto const jweight = v - j;

				for (auto k = 0; k < 2; ++k) {
					auto const kterm = k * ww + (1.0 - k) * (1.0 - ww);
					auto const kweight = w - k;

					auto const weight_v = Vec3{iweight, jweight, kweight};
					accum += iterm * jterm * kterm * dot(c[i][j][k], weight_v);
				}
			}
		}
		return accum;
	}
}

Perlin::Perlin(std::default_random_engine &rand_eng):
	ranvec_{rand_fill(rand_eng)},
	perm_x_{perlin_generate_perm(rand_eng)},
	perm_y_{perlin_generate_perm(rand_eng)},
	perm_z_{perlin_generate_perm(rand_eng)}
{
}

auto Perlin::noise(Vec3 const p) const noexcept -> double {
	constexpr auto mask = Perlin::point_count - 1;

	auto const fpx = std::floor(p.x);
	auto const fpy = std::floor(p.y);
	auto const fpz = std::floor(p.z);

	auto const u = p.x - fpx;
	auto const v = p.y - fpy;
	auto const w = p.z - fpz;

	auto const i = static_cast<int>(fpx);
	auto const j = static_cast<int>(fpy);
	auto const k = static_cast<int>(fpz);

	std::array<std::array<std::array<Vec3, 2>, 2>, 2> c;

	for (auto di = 0; di < 2; ++di) {
		auto const iterm = static_cast<std::size_t>(i + di) & mask;
		assert(iterm < perm_x_.size());
		auto const xterm = perm_x_[iterm];

		for (auto dj = 0; dj < 2; ++dj) {
			auto const jterm = static_cast<std::size_t>(j + dj) & mask;
			assert(jterm < perm_y_.size());
			auto const yterm = perm_y_[jterm];

			for (auto dk = 0; dk < 2; ++dk) {
				auto const kterm = static_cast<std::size_t>(k + dk) & mask;
				assert(kterm < perm_z_.size());
				auto const zterm = perm_z_[kterm];

				auto const ind = xterm ^ yterm ^ zterm;
				assert(ind < ranvec_.size());
				c[di][dj][dk] = ranvec_[ind];
			}
		}
	}

	return perlin_interp(c, u, v, w);
}

auto Perlin::turb(Vec3 const p, int const depth) const noexcept -> double {
	auto accum = 0.0;
	auto temp_p = p;
	auto weight = 1.0;

	for (auto i = 0; i < depth; ++i) {
		accum += weight * noise(temp_p);
		weight *= 0.5;
		temp_p *= 2.0;
	}

	return std::abs(accum);
}
