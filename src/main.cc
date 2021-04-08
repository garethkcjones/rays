#include <cstdlib>
#include <exception>
#include <filesystem>
#include <iostream>
#include <string>

namespace fs = std::filesystem;

namespace {
	/*
	 * Runs the program.
	 */
	void run(int const /*argc*/, char const *const *const /*argv*/) {
		// FIXME: Implement.
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
 * Usage: rays
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
