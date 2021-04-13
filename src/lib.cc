#include "lib.hh"

#include <cassert>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <limits>
#include <ostream>
#include <random>

#include "camera.hh"
#include "colour.hh"
#include "hittable.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays;
using hittable::Hittable;

namespace {
	/*
	 * Calculates the colour of a ray of light.
	 */
	auto ray_colour(Ray const &r,
	                Hittable const &world,
	                std::default_random_engine &rand_eng)
		-> Colour
	{
		constexpr auto infinity = std::numeric_limits<double>::infinity();

		if (auto const rec = world.hit(r, 0.0, infinity); rec) {
			auto const target = rec->p() + rec->normal()
			                   + Vec3::new_random_in_unit_sphere(rand_eng);
			return 0.5 * ray_colour(Ray{rec->p(), target - rec->p()}, world,
			                        rand_eng);
		}

		auto const unit_direction = r.direction().unit();
		auto const t = 0.5 * (unit_direction.y + 1.0);
		return (1.0 - t) * Colour{1.0, 1.0, 1.0} + t * Colour{0.5, 0.7, 1.0};
	}
}

/*
 * Renders a scene.
 *
 * # Parameters
 *
 * * `world` contains the hittable objects in the scene.
 * * `image_width` and `image_height` are the image dimesions, in pixels.
 * * `samples_per_pixel` is the number of samples per pixel.
 * * `cam` is the camera.
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
void rays::render(Hittable const &world,
                  int const image_width,
                  int const image_height,
                  int const samples_per_pixel,
                  Camera const &cam,
                  std::ostream &output,
                  bool const log)
{
	assert(image_width > 1);
	assert(image_height > 1);
	assert(samples_per_pixel > 0);

	// Initialize random number generator.
	auto rand_dev = std::random_device{};
	auto rand_eng = std::default_random_engine{rand_dev()};
	auto rand_dst = std::uniform_real_distribution{0.0, 1.0};

	// Render.

	auto const width_scale = static_cast<double>(image_width - 1);
	auto const height_scale = static_cast<double>(image_height - 1);

	output << "P3\n" << image_width << ' ' << image_height << "\n255\n";

	for (auto j = image_height - 1; j >= 0; --j) {
		if (log) {
			auto const percent =
				std::round(100.0 * static_cast<double>(image_height - j)
				           / image_height);
			std::cerr << "\rScanlines remaining: " << std::setw(5) << j
			          << "   (" << std::setw(3) << percent << " % complete)"
			          << std::flush;
		}

		for (auto i = 0; i < image_width; ++i) {
			auto pixel_colour = Colour{0.0, 0.0, 0.0};

			for (auto s = 0; s < samples_per_pixel; ++s) {
				auto const ur = rand_dst(rand_eng);
				auto const vr = rand_dst(rand_eng);

				auto const u = (static_cast<double>(i) + ur) / width_scale;
				auto const v = (static_cast<double>(j) + vr) / height_scale;

				auto const r = cam.get_ray(u, v);

				pixel_colour += ray_colour(r, world, rand_eng);
			}

			auto const [ir, ig, ib] = pixel_colour.to_rgb8(samples_per_pixel);

			output << int{ir} << ' ' << int{ig} << ' ' << int{ib} << '\n';
		}
	}

	if (log)
		std::cerr << "\nDone.\n";
}
