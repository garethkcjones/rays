#include "texture_chequer.hh"

#include <cassert>
#include <cmath>
#include <memory>
#include <utility>

#include "colour.hh"
#include "texture.hh"
#include "texture_solidcolour.hh"
#include "vec3.hh"

using namespace rays::texture;

Chequer::Chequer(std::shared_ptr<Texture> even,
                 std::shared_ptr<Texture> odd) noexcept:
	even_{std::move(even)},
	odd_{std::move(odd)}
{
	assert(even_);
	assert(odd_);
}

Chequer::Chequer(Colour const even, Colour const odd):
	Chequer{SolidColour::new_texture(even), SolidColour::new_texture(odd)}
{
}

auto Chequer::new_texture(std::shared_ptr<Texture> even,
                          std::shared_ptr<Texture> odd)
	-> std::shared_ptr<Texture>
{
	return std::make_shared<Chequer>(std::move(even), std::move(odd));
}

auto Chequer::new_texture(Colour const even, Colour const odd)
	-> std::shared_ptr<Texture>
{
	return std::make_shared<Chequer>(even, odd);
}

auto Chequer::value(double const u, double const v, Vec3 const p) const
	-> Colour
{
	using std::sin;

	auto const sines = sin(10.0 * p.x) * sin(10.0 * p.y) * sin(10.0 * p.z);

	if (sines < 0.0)
		return odd_->value(u, v, p);
	else
		return even_->value(u, v, p);
}
