#include "hittable_aabb.hh"

#include <algorithm>
#include <utility>

#include "ray.hh"

using namespace rays::hittable;

namespace {
	auto hit1d(double const minimum,
	           double const maximum,
	           double const origin,
	           double const direction,
	           double &t_min,
	           double &t_max)
		-> bool
	{
		auto const inv_dir = 1.0 / direction;
		auto const [t0, t1] = std::minmax(
			(minimum - origin) * inv_dir,
			(maximum - origin) * inv_dir
		);

		t_min = std::max(t0, t_min);
		t_max = std::min(t1, t_max);

		return t_min < t_max;
	}
}

auto Aabb::hit(Ray const &r, double t_min, double t_max) const noexcept -> bool
{
	auto const [mnx, mny, mnz] = minimum_;
	auto const [mxx, mxy, mxz] = maximum_;

	auto const [ox, oy, oz] = r.origin();
	auto const [dx, dy, dz] = r.direction();

	return    hit1d(mnx, mxx, ox, dx, t_min, t_max)
	       && hit1d(mny, mxy, oy, dy, t_min, t_max)
	       && hit1d(mnz, mxz, oz, dz, t_min, t_max);
}

auto Aabb::surrounding_box(Aabb const box0, Aabb const box1) noexcept -> Aabb {
	auto const [b0mnx, b0mny, b0mnz] = box0.minimum_;
	auto const [b0mxx, b0mxy, b0mxz] = box0.maximum_;
	auto const [b1mnx, b1mny, b1mnz] = box1.minimum_;
	auto const [b1mxx, b1mxy, b1mxz] = box1.maximum_;

	auto const small = Vec3 {
		std::min(b0mnx, b1mnx),
		std::min(b0mny, b1mny),
		std::min(b0mnz, b1mnz)
	};

	auto const big = Vec3 {
		std::max(b0mxx, b1mxx),
		std::max(b0mxy, b1mxy),
		std::max(b0mxz, b1mxz)
	};

	return Aabb{small, big};
}
