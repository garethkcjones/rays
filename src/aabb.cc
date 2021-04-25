#include "aabb.hh"

#include <algorithm>
#include <utility>

#include "ray.hh"

using namespace rays;

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
