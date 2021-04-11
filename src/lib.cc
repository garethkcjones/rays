#include "lib.hh"

#include <cmath>
#include <iomanip>
#include <iostream>
#include <limits>
#include <ostream>

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
	auto ray_colour(Ray const &r, Hittable const &world) noexcept -> Colour {
		constexpr auto infinity = std::numeric_limits<double>::infinity();

		if (auto const rec = world.hit(r, 0.0, infinity); rec) {
			auto const [x, y, z] = rec->normal();
			return 0.5 * Colour{x + 1.0, y + 1.0, z + 1.0};
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
 * * `viewport_width` and `viewport_height` are the viewport dimensions, in
 *    virtual co-ordinates.
 * * `focal_length` is the camera focal length.
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
void rays::render(Hittable const &world,
                  int const image_width,
                  int const image_height,
                  double const viewport_width,
                  double const viewport_height,
                  double const focal_length,
                  std::ostream &output,
                  bool const log)
{
	// Geometry.

	constexpr auto origin = Vec3{0.0, 0.0, 0.0};
	auto const horizontal = Vec3{viewport_width, 0.0, 0.0};
	auto const vertical = Vec3{0.0, viewport_height, 0.0};
	auto const lower_left_corner =
		origin - 0.5 * (horizontal + vertical) - Vec3{0.0, 0.0, focal_length};

	// Render.

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
			auto const u = static_cast<double>(i) / (image_width - 1);
			auto const v = static_cast<double>(j) / (image_height - 1);

			auto const r = Ray {
				origin,
				lower_left_corner + u * horizontal + v * vertical - origin
			};

			auto const pixel_colour = ray_colour(r, world);
			auto const [ir, ig, ib] = pixel_colour.to_rgb8();

			output << int{ir} << ' ' << int{ig} << ' ' << int{ib} << '\n';
		}
	}

	if (log)
		std::cerr << "\nDone.\n";
}
