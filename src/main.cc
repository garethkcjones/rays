#include <cstddef>
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
#include "texture_chequer.hh"
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
		using rays::texture::Chequer;
		using rays::Vec3;

		auto rand_dev = std::random_device{};
		auto rand_eng = std::default_random_engine{rand_dev()};
		auto rand_dst = std::uniform_real_distribution{0.0, 1.0};

		auto const world = std::make_shared<HittableList>();

		auto chequer = Chequer::new_texture(
			Vec3{10.0, 10.0, 10.0},
			Colour{0.2, 0.3, 0.1},
			Colour{0.9, 0.9, 0.9}
		);
		auto ground_material = Lambertian2::new_material(std::move(chequer));
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

	auto two_spheres() -> std::shared_ptr<rays::hittable::HittableList> {
		using rays::Colour;
		using rays::hittable::HittableList;
		using rays::hittable::Sphere;
		using rays::material::Lambertian2;
		using rays::texture::Chequer;
		using rays::Vec3;

		auto const objects = std::make_shared<HittableList>();

		auto chequer = Chequer::new_texture(
			Vec3{10.0, 10.0, 10.0},
			Colour{0.2, 0.3, 0.1},
			Colour{0.9, 0.9, 0.9}
		);

		objects->push_back(Sphere::new_hittable(Vec3{0.0, -10.0, 0.0}, 10.0,
			Lambertian2::new_material(chequer)));
		objects->push_back(Sphere::new_hittable(Vec3{0.0, 10.0, 0.0}, 10.0,
			Lambertian2::new_material(std::move(chequer))));

		return objects;
	}

	/*
	 * Builds and renders a scene.
	 */
	void render(int const scene, std::ostream &output) {
		using rays::Camera;
		using rays::hittable::Hittable;
		using rays::Vec3;

		// Scene parameters.
		std::shared_ptr<Hittable> world;
		Vec3 lookfrom, lookat, vup;
		double vfov, aspect_ratio, aperture, dist_to_focus, time0, time1;
		int image_width, image_height, samples_per_pixel, max_depth;

		switch (scene) {
			case 1: {
				// Image.
				auto const image_aspect_ratio = 16.0 / 9.0;
				image_width = 400;
				image_height =
					static_cast<int>(image_width / image_aspect_ratio);
				samples_per_pixel = 100;
				max_depth = 50;

				// World.
				world = random_scene();

				// Camera.
				lookfrom = Vec3{13.0, 2.0, 3.0};
				lookat   = Vec3{ 0.0, 0.0, 0.0};
				vup      = Vec3{ 0.0, 1.0, 0.0};
				vfov = 20.0;
				aspect_ratio = static_cast<double>(image_width) / image_height;
				aperture = 0.1;
				dist_to_focus = 10.0;
				time0 = 0.0;
				time1 = 1.0;

				break;
			}

			case 2: {
				// Image.
				auto const image_aspect_ratio = 16.0 / 9.0;
				image_width = 400;
				image_height =
					static_cast<int>(image_width / image_aspect_ratio);
				samples_per_pixel = 100;
				max_depth = 50;

				// World.
				world = two_spheres();

				// Camera.
				lookfrom = Vec3{13.0, 2.0, 3.0};
				lookat   = Vec3{ 0.0, 0.0, 0.0};
				vup      = Vec3{ 0.0, 1.0, 0.0};
				vfov = 20.0;
				aspect_ratio = static_cast<double>(image_width) / image_height;
				aperture = 0.0;
				dist_to_focus = 10.0;
				time0 = 0.0;
				time1 = 1.0;

				break;
			}

			default:
				throw std::runtime_error {
					"invalid scene number: " + std::to_string(scene)
				};
		}

		auto const cam = std::make_shared<Camera>(
			lookfrom,
			lookat,
			vup,
			vfov,
			aspect_ratio,
			aperture,
			dist_to_focus,
			time0,
			time1
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

	auto scene_number(std::string const arg) -> int {
		std::size_t pos;
		auto const scene = std::stoi(arg, &pos);
		if (pos == arg.length())
			return scene;
		throw std::runtime_error{"invalid scene number “" + arg + "”"};
	}

	/*
	 * Runs the program.
	 */
	void run(int const argc, char const *const *const argv) {
		switch (argc) {
			case 0:
			case 1:
				throw std::runtime_error{"no scene number specified"};

			case 2: {
				auto const scene = scene_number(argv[1]);

				// No output file name specified on command-line.  Use stdout.
				std::ios::sync_with_stdio(false);
				render(scene, std::cout);
				break;
			}

			case 3: {
				auto const scene = scene_number(argv[1]);

				// Get the output file name from the command-line.
				auto const filename = fs::path{argv[2]};

				auto output = std::ofstream{filename};
				if (!output) {
					throw std::runtime_error {
						"cannot open output file “" + filename.string() + "”"
					};
				}

				render(scene, output);

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
