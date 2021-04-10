#include "lib.hh"

#include <cmath>
#include <iomanip>
#include <iostream>
#include <ostream>

#include "colour.hh"

/*
 * Runs the program.
 *
 * # Parameters
 *
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
void rays::run(std::ostream &output, bool const log) {
	// Image

	constexpr auto image_width = 256;
	constexpr auto image_height = 256;

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
			auto const r = static_cast<double>(i) / (image_width - 1);
			auto const g = static_cast<double>(j) / (image_height - 1);
			auto const b = 0.25;

			auto const pixel_colour = Colour{r, g, b};
			auto const [ir, ig, ib] = pixel_colour.to_rgb8();

			output << int{ir} << ' ' << int{ig} << ' ' << int{ib} << '\n';
		}
	}

	if (log)
		std::cerr << "\nDone.\n";
}
