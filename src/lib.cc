#include "lib.hh"

#include <cmath>
#include <iomanip>
#include <iostream>
#include <ostream>

#include "colour.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays;

namespace {
	auto hit_sphere(Vec3 const centre, double const radius, Ray const &r)
		noexcept -> double
	{
		auto const oc = r.origin() - centre;
		auto const a = dot(r.direction(), r.direction());
		auto const half_b = dot(oc, r.direction());
		auto const c = dot(oc, oc) - radius * radius;
		auto const discriminant = half_b * half_b - a * c;
		if (discriminant < 0.0)
			return -1.0;
		else
			return (-half_b - std::sqrt(discriminant)) / a;
	}

	/*
	 * Calculates the colour of a ray of light.
	 */
	auto ray_colour(Ray const &r) noexcept -> Colour {
		auto t = hit_sphere(Vec3{0.0, 0.0, -1.0}, 0.5, r);
		if (t > 0.0) {
			auto const n = (r.at(t) - Vec3{0.0, 0.0, -1.0}).unit();
			return 0.5 * Colour{n.x + 1.0, n.y + 1.0, n.z + 1.0};
		}
		auto const unit_direction = r.direction().unit();
		t = 0.5 * (unit_direction.y + 1.0);
		return (1.0 - t) * Colour{1.0, 1.0, 1.0} + t * Colour{0.5, 0.7, 1.0};
	}
}

/*
 * Renders a scene.
 *
 * # Parameters
 *
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
void rays::render(std::ostream &output, bool const log) {
	// Image

	constexpr auto image_aspect_ratio = 16.0 / 9.0;
	constexpr auto image_width = 400;
	constexpr auto image_height =
		static_cast<int>(image_width / image_aspect_ratio);

	// Camera

	constexpr auto viewport_aspect_ratio =
		static_cast<double>(image_width) / image_height;
	constexpr auto viewport_height = 2.0;
	constexpr auto viewport_width = viewport_aspect_ratio * viewport_height;
	constexpr auto focal_length = 1.0;

	constexpr auto origin = Vec3{0.0, 0.0, 0.0};
	constexpr auto horizontal = Vec3{viewport_width, 0.0, 0.0};
	constexpr auto vertical = Vec3{0.0, viewport_height, 0.0};
	constexpr auto lower_left_corner =
		origin - 0.5 * (horizontal + vertical) - Vec3{0.0, 0.0, focal_length};

	// Render

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

			auto const pixel_colour = ray_colour(r);
			auto const [ir, ig, ib] = pixel_colour.to_rgb8();

			output << int{ir} << ' ' << int{ig} << ' ' << int{ib} << '\n';
		}
	}

	if (log)
		std::cerr << "\nDone.\n";
}
