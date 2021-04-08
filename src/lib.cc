#include "lib.hh"

#include <ostream>

/*
 * Runs the program.
 *
 * # Parameters
 *
 * * `output` is the stream to write the generated image to.
 */
void rays::run(std::ostream &output) {
	// Image

	constexpr auto image_width = 256;
	constexpr auto image_height = 256;

	// Render

	output << "P3\n" << image_width << ' ' << image_height << "\n255\n";

	for (auto j = image_height - 1; j >= 0; --j) {
		for (auto i = 0; i < image_width; ++i) {
			auto const r = static_cast<double>(i) / (image_width - 1);
			auto const g = static_cast<double>(j) / (image_height - 1);
			auto const b = 0.25;

			auto const ir = static_cast<int>(255.999 * r);
			auto const ig = static_cast<int>(255.999 * g);
			auto const ib = static_cast<int>(255.999 * b);

			output << ir << ' ' << ig << ' ' << ib << '\n';
		}
	}
}
