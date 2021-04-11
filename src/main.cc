#include <cstdlib>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>

#include "lib.hh"

namespace fs = std::filesystem;

namespace {
	/*
	 * Builds and renders a scene.
	 */
	void render(std::ostream &output) {
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

		// Render.

		rays::render(image_width, image_height, viewport_width, viewport_height,
		             focal_length, output, true);
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
