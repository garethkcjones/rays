#include "hittable_constantmedium.hh"

#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>
#include <limits>
#include <memory>
#include <optional>
#include <random>
#include <utility>

#include "colour.hh"
#include "hittable.hh"
#include "hittable_aabb.hh"
#include "hittable_hitrecord.hh"
#include "material_isotropic.hh"
#include "ray.hh"
#include "texture.hh"
#include "texture_solidcolour.hh"
#include "vec3.hh"

using namespace rays::hittable;
using rays::material::Isotropic;
using rays::texture::SolidColour;
using rays::texture::Texture;

ConstantMedium::ConstantMedium(std::shared_ptr<Hittable> boundary,
                               double const density,
                               std::shared_ptr<Texture> albedo):
	boundary_{std::move(boundary)},
	phase_function_{Isotropic::new_material(std::move(albedo))},
	neg_inv_density_{-1.0 / density}
{
	assert(boundary_);
	assert(phase_function_);
}

auto ConstantMedium::new_hittable(std::shared_ptr<Hittable> boundary,
                                  double const density,
                                  std::shared_ptr<Texture> albedo)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<ConstantMedium>(std::move(boundary),
	                                        density,
	                                        std::move(albedo));
}

auto ConstantMedium::new_hittable(std::shared_ptr<Hittable> boundary,
                                  double const density,
                                  Colour albedo)
	-> std::shared_ptr<Hittable>
{
	return new_hittable(std::move(boundary),
	                    density,
	                    SolidColour::new_texture(albedo));
}

auto ConstantMedium::hit(Ray const &r,
                         double const t_min,
                         double const t_max,
                         std::default_random_engine &rand_eng) const
	-> std::optional<HitRecord>
{
	// Print occasional samples when debugging. To enable, set to `true`.
	constexpr auto enable_debug = false;

	constexpr auto infinity = std::numeric_limits<double>::infinity();

	auto rand_dst = std::uniform_real_distribution{0.0, 1.0};
	auto const debugging = enable_debug && rand_dst(rand_eng) < 0.00001;

	if (auto const rec1 = boundary_->hit(r, -infinity, infinity, rand_eng);
	    rec1)
	{
		if (auto const rec2 = boundary_->hit(r, rec1->t() + 0.0001, infinity,
		                                     rand_eng);
		    rec2)
		{

			if (debugging) {
				std::cerr << "\nt_min = " << rec1->t()
				          << ", t_max = " << rec2->t() << '\n';
			}

			auto t1 = std::max(rec1->t(), t_min);
			auto t2 = std::min(rec2->t(), t_max);

			if (t1 < t2) {
				t1 = std::max(t1, 0.0);

				auto const ray_length = r.direction().length();
				auto const distance_inside_boundary = (t2 - t1) * ray_length;
				auto const hit_distance =
					neg_inv_density_ * std::log(rand_dst(rand_eng));

				if (hit_distance <= distance_inside_boundary) {
					auto const t = t1 + hit_distance / ray_length;
					auto const p = r.at(t);

					if (debugging) {
						std::cerr << "hit_distance = " <<  hit_distance << '\n'
						          << "rec.t = " << t << '\n'
						          << "rec.p = " << p << '\n';
					}

					auto const normal = Vec3{1.0, 0.0, 0.0};  // Arbitrary.
					auto const u = 0.0;                       // Arbitrary.
					auto const v = 0.0;                       // Arbitrary.

					return std::make_optional<HitRecord>(r, p, normal, t, u, v,
						phase_function_);
				}
			}
		}
	}

	return std::nullopt;
}

auto ConstantMedium::bounding_box(double const time0, double const time1) const
	-> Aabb
{
	return boundary_->bounding_box(time0, time1);
}
