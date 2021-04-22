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

Chequer::Chequer(Vec3 const scale,
                 std::shared_ptr<Texture> even,
                 std::shared_ptr<Texture> odd) noexcept:
	scale_{scale},
	even_{std::move(even)},
	odd_{std::move(odd)}
{
	assert(even_);
	assert(odd_);
}

auto Chequer::new_texture(Vec3 const scale,
                          std::shared_ptr<Texture> even,
                          std::shared_ptr<Texture> odd)
	-> std::shared_ptr<Texture>
{
	return std::make_shared<Chequer>(scale, std::move(even), std::move(odd));
}

auto Chequer::new_texture(Vec3 const scale, Colour const even, Colour const odd)
	-> std::shared_ptr<Texture>
{
	return new_texture(scale,
	                   SolidColour::new_texture(even),
	                   SolidColour::new_texture(odd));
}

auto Chequer::value(double const u, double const v, Vec3 p) const
	-> Colour
{
	using std::sin;

	p *= scale_;
	auto const sines = sin(p.x) * sin(p.y) * sin(p.z);

	if (sines < 0.0)
		return odd_->value(u, v, p);
	else
		return even_->value(u, v, p);
}
