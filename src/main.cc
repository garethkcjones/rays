#include <cstdlib>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <random>
#include <stdexcept>
#include <string>
#include <utility>

#include "camera.hh"
#include "hittable.hh"
#include "hittable_sphere.hh"
#include "lib.hh"
#include "material_dielectric.hh"
#include "material_lambertian.hh"
#include "material_metal.hh"
#include "vec3.hh"

namespace fs = std::filesystem;

namespace {
	auto random_scene() -> std::shared_ptr<rays::hittable::HittableList> {
		using rays::Colour;
		using rays::hittable::HittableList;
		using rays::hittable::MovingSphere;
		using rays::hittable::Sphere;
		using rays::material::Dielectric;
		using rays::material::Lambertian2;
		using rays::material::Metal;
		using rays::Vec3;

		auto rand_dev = std::random_device{};
		auto rand_eng = std::default_random_engine{rand_dev()};
		auto rand_dst = std::uniform_real_distribution{0.0, 1.0};

		auto const world = std::make_shared<HittableList>();

		auto ground_material = Lambertian2::new_material(Colour{0.5, 0.5, 0.5});
		world->push_back(Sphere::new_hittable(Vec3{0.0, -1000.0, 0.0}, 1000.0,
			std::move(ground_material)));

		for (auto a = -11; a < 11; ++a) {
			for (auto b = -11; b < 11; ++b) {
				auto const choose_mat = rand_dst(rand_eng);
				auto const centre = Vec3 {
					a + 0.9 * rand_dst(rand_eng),
					0.2,
					b + 0.9 * rand_dst(rand_eng)
				};

				if ((centre - Vec3{4.0, 0.2, 0.0}).length() > 0.9) {
					if (choose_mat < 0.8) {
						// Diffuse.
						auto const albedo =
							  Colour::new_random(rand_eng, 0.0, 1.0)
							* Colour::new_random(rand_eng, 0.0, 1.0);
						auto sphere_material =
							Lambertian2::new_material(albedo);
						auto const centre2 =
							centre + Vec3{0.0, 0.5 * rand_dst(rand_eng), 0.0};
						world->push_back(MovingSphere::new_hittable(
							centre,
							centre2,
							0.0,
							1.0,
							0.2,
							std::move(sphere_material)
						));
					} else if (choose_mat < 0.95) {
						// Metal.
						auto const albedo =
							Colour::new_random(rand_eng, 0.5, 1.0);
						auto const fuzz = 0.5 * rand_dst(rand_eng);
						auto sphere_material =
							Metal::new_material(albedo, fuzz);
						world->push_back(Sphere::new_hittable(centre, 0.2,
							std::move(sphere_material)));
					} else {
						// Glass.
						auto sphere_material = Dielectric::new_material(1.5);
						world->push_back(Sphere::new_hittable(centre, 0.2,
							std::move(sphere_material)));
					}
				}
			}
		}

		auto material1 = Dielectric::new_material(1.5);
		world->push_back(Sphere::new_hittable(Vec3{0.0, 1.0, 0.0}, 1.0,
			std::move(material1)));

		auto material2 = Lambertian2::new_material(Colour{0.4, 0.2, 0.1});
		world->push_back(Sphere::new_hittable(Vec3{-4.0, 1.0, 0.0}, 1.0,
			std::move(material2)));

		auto material3 = Metal::new_material(Colour{0.7, 0.6, 0.5}, 0.0);
		world->push_back(Sphere::new_hittable(Vec3{4.0, 1.0, 0.0}, 1.0,
			std::move(material3)));

		return world;
	}

	/*
	 * Builds and renders a scene.
	 */
	void render(std::ostream &output) {
		using rays::Camera;
		using rays::Vec3;

		// Image.

		constexpr auto image_aspect_ratio = 3.0 / 2.0;
		constexpr auto image_width = 1200;
		constexpr auto image_height =
			static_cast<int>(image_width / image_aspect_ratio);
		constexpr auto samples_per_pixel = 500;
		constexpr auto max_depth = 50;

		// World.

		auto const world = random_scene();

		// Camera

		constexpr auto lookfrom = Vec3{13.0, 2.0, 3.0};
		constexpr auto lookat   = Vec3{ 0.0, 0.0, 0.0};
		constexpr auto vup      = Vec3{ 0.0, 1.0, 0.0};
		constexpr auto vfov = 20.0;
		constexpr auto aspect_ratio =
			static_cast<double>(image_width) / image_height;
		constexpr auto aperture = 0.1;
		constexpr auto dist_to_focus = 10.0;

		auto const cam = std::make_shared<Camera>(
			lookfrom,
			lookat,
			vup,
			vfov,
			aspect_ratio,
			aperture,
			dist_to_focus
		);

		// Render.

		auto const num_threads = 32;

		rays::run(
			num_threads,
			std::move(world),
			image_width,
			image_height,
			samples_per_pixel,
			max_depth,
			std::move(cam),
			output,
			true
		);
	}

	/*
	 * Runs the program.
	 */
	void run(int const argc, char const *const *const argv) {
		switch (argc) {
			case 0:
			case 1:
				// No output file name specified on command-line.  Use stdout.
				std::ios::sync_with_stdio(false);
				render(std::cout);
				break;

			case 2: {
				// Get the output file name from the command-line.
				auto const filename = fs::path{argv[1]};

				auto output = std::ofstream{filename};
				if (!output) {
					throw std::runtime_error {
						"cannot open output file “" + filename.string() + "”"
					};
				}

				render(output);

				if (!output.flush()) {
					throw std::runtime_error {
						"error writing to “" + filename.string() + "”"
					};
				}

				break;
			}

			default:
				throw std::runtime_error{"too many command-line arguments"};
		}
	}

	/*
	 * Returns the program name from the command-line.
	 */
	auto get_progname(int const argc, char const *const *const argv)
		-> std::string
	{
		if (argc && argv && *argv && **argv) {
			auto const progname = fs::path{argv[0]}.filename().string();
			if (!progname.empty())
				return progname;
		}
		return "rays";
	}
}

/*
 * Entry point.
 *
 * Usage: rays [OUTPUT_FILE]
 */
int main(int const argc, char const *const *const argv) {
	auto const progname = get_progname(argc, argv);

	try {
		run(argc, argv);
	} catch (std::exception const &x) {
		std::cerr << progname << ": " << x.what() << '\n';
		return EXIT_FAILURE;
	}

	return EXIT_SUCCESS;
}
