#include "lib.hh"

#include <cassert>
#include <cmath>
#include <future>
#include <iomanip>
#include <iostream>
#include <limits>
#include <memory>
#include <ostream>
#include <random>
#include <utility>
#include <vector>

#include "camera.hh"
#include "colour.hh"
#include "hittable.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays;
using hittable::Hittable;

namespace {
	/*
	 * Calculates the colour of a ray of light.
	 */
	auto ray_colour(Ray const &r,
	                Colour const background,
	                Hittable const &world,
	                int const depth,
	                std::default_random_engine &rand_eng)
		-> Colour
	{
		constexpr auto infinity = std::numeric_limits<double>::infinity();

		// If we’ve exceeded the ray bounce limit, no more light is gathered.
		if (depth <= 0)
			return Colour{0.0, 0.0, 0.0};

		if (auto const rec = world.hit(r, 0.001, infinity, rand_eng); rec) {
			auto const emitted =
				rec->material_ref()
				.emitted(rec->u(), rec->v(), rec->p(), rand_eng);

			if (auto const s = rec->material_ref().scatter(r, *rec, rand_eng);
			    s)
			{
				auto const [attenuation, scattered] = *s;
				return emitted + attenuation *
				  ray_colour(scattered, background, world, depth - 1, rand_eng);
			} else {
				return emitted;
			}
		} else {
			// If the ray hits nothing, return the background colour.
			return background;
		}
	}

	/*
	 * Renders a scene.
	 *
	 * # Parameters
	 *
	 * * `world` contains the hittable objects in the scene.
	 * * `background` is the background colour.
	 * * `image_width` and `image_height` are the image dimesions, in pixels.
	 * * `samples_per_pixel` is the number of samples per pixel.
	 * * `max_depth` is the recursion limit for ray reflections.
	 * * `cam` is the camera.
	 * * If `log` is `true`, progress is reported to the standard error stream.
	 */
	auto render(std::shared_ptr<Hittable const> const world,
	            Colour const background,
	            int const image_width,
	            int const image_height,
	            int const samples_per_pixel,
	            int const max_depth,
	            std::shared_ptr<Camera const> const cam,
	            bool const log)
		-> std::vector<Colour>
	{
		assert(world);
		assert(image_width > 1);
		assert(image_height > 1);
		assert(samples_per_pixel > 0);
		assert(max_depth > 0);
		assert(cam);

		// Initialize random number generator.
		auto rand_dev = std::random_device{};
		auto rand_eng = std::default_random_engine{rand_dev()};
		auto rand_dst = std::uniform_real_distribution{0.0, 1.0};

		// Render.

		std::vector<Colour> pixels;
		pixels.reserve(image_width * image_height);

		auto const width_scale = static_cast<double>(image_width - 1);
		auto const height_scale = static_cast<double>(image_height - 1);

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

					auto const u = (i + ur) / width_scale;
					auto const v = (j + vr) / height_scale;

					auto const r = cam->get_ray(u, v, rand_eng);

					pixel_colour +=
						ray_colour(r, background, *world, max_depth, rand_eng);
				}

				pixels.push_back(pixel_colour);
			}
		}

		if (log)
			std::cerr << '\n';

		return pixels;
	}

	/*
	 * Writes an image file.
	 *
	 * # Parameters
	 *
	 * * `output` is the stream to write the generated image to.
	 * * `pixels` is the image data.
	 * * `image_width` and `image_height` are the image dimesions, in pixels.
	 * * `samples_per_pixel` is the number of samples per pixel.
	 * * If `log` is `true`, progress is reported to the standard error stream.
	 */
	void write_file(std::ostream &output,
	                std::vector<Colour> const &pixels,
	                int const image_width,
	                int const image_height,
	                int const samples_per_pixel,
	                bool const log)
	{
		assert(image_width > 1);
		assert(image_height > 1);
		assert(samples_per_pixel > 0);

		if (log)
			std::cerr << "Writing output...\n";

		output << "P3\n" << image_width << ' ' << image_height << "\n255\n";
		for (auto &&pixel_colour : pixels) {
			auto const [ir, ig, ib] = pixel_colour.to_rgb8(samples_per_pixel);
			output << int{ir} << ' ' << int{ig} << ' ' << int{ib} << '\n';
		}

		if (log)
			std::cerr << "Done.\n";
	}
}

/*
 * Runs the program.
 *
 * # Parameters
 *
 * * `num_threads` is the number of threads to distribute rendering over.
 * * `world` contains the hittable objects in the scene.
 * * `background` is the background colour.
 * * `image_width` and `image_height` are the image dimesions, in pixels.
 * * `samples_per_pixel` is the number of samples per pixel.
 * * `max_depth` is the recursion limit for ray reflections.
 * * `cam` is the camera.
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
void rays::run(int const num_threads,
               std::shared_ptr<Hittable const> world,
               Colour const background,
               int const image_width,
               int const image_height,
               int const samples_per_pixel,
               int const max_depth,
               std::shared_ptr<Camera const> cam,
               std::ostream &output,
               bool const log)
{
	assert(num_threads > 0);

	auto const samples_per_thread = samples_per_pixel / num_threads;
	auto const remaining_samples = samples_per_pixel % num_threads;

	// Spawn threads.
	std::vector<std::future<std::vector<Colour>>> threads;
	threads.reserve(num_threads - 1);
	for (auto thread_num = 1; thread_num < num_threads; ++thread_num) {
		auto const samples_per_pixel =   thread_num <= remaining_samples
		                               ? samples_per_thread + 1
		                               : samples_per_thread;
		threads.push_back(std::async(
			std::launch::async,
			render,
			world,
			background,
			image_width,
			image_height,
			samples_per_pixel,
			max_depth,
			cam,
			false
		));
	}

	// This thread.
	auto pixels = render(
		std::move(world),
		background,
		image_width,
		image_height,
		samples_per_thread,
		max_depth,
		std::move(cam),
		log
	);

	// Join threads.
	auto i = 1;
	for (auto &&thread : threads) {
		if (log) {
			std::cerr << "\rWaiting for thread " << std::setw(2) << ++i
			          << " of " << num_threads << "...";
			auto const thread_pixels = thread.get();
			assert(pixels.size() == thread_pixels.size());
			for (auto i = decltype(pixels.size()){0}; i < pixels.size(); ++i)
				pixels[i] += thread_pixels[i];
		}
	}
	if (log)
		std::cerr << '\n';

	write_file(
		output,
		pixels,
		image_width,
		image_height,
		samples_per_pixel,
		log
	);
}
