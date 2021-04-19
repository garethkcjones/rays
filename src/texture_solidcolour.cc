#include "texture_solidcolour.hh"

#include <memory>

#include "colour.hh"
#include "texture.hh"
#include "vec3.hh"

using namespace rays::texture;

SolidColour::SolidColour(Colour const value) noexcept:
	value_{value}
{
}

auto SolidColour::new_texture(Colour const value) -> std::shared_ptr<Texture> {
	return std::make_shared<SolidColour>(value);
}

auto SolidColour::value(double /*u*/, double /*v*/, Vec3 /*p*/) const -> Colour
{
	return value_;
}
