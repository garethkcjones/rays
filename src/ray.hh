#pragma once

#include "vec3.hh"

namespace rays {
	class Ray;
}

/*
 * Type to represent a ray of light.
 */
class rays::Ray final {
	public:

		constexpr explicit Ray(Vec3 origin, Vec3 direction, double time = 0.0)
			noexcept;

		constexpr auto origin() const noexcept {return origin_;}
		constexpr auto direction() const noexcept {return direction_;}
		constexpr auto time() const noexcept {return time_;}

		constexpr Vec3 at(double t) const noexcept;

	private:

		Vec3 origin_;
		Vec3 direction_;
		double time_;
};

inline constexpr rays::Ray::Ray(Vec3 const origin,
                                Vec3 const direction,
                                double const time) noexcept:
	origin_{origin},
	direction_{direction},
	time_{time}
{
}

inline constexpr auto rays::Ray::at(double const t) const noexcept -> Vec3 {
	return origin_ + t * direction_;
}
